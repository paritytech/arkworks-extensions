use ark_ff::{Field, MontFp, PrimeField, Zero};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, SerializationError};
use ark_std::{marker::PhantomData, ops::Neg, vec::Vec, One};
use codec::{Decode, Encode};
use sp_ark_models::{
    bls12,
    bls12::Bls12Config,
    short_weierstrass::{Affine, Projective, SWCurveConfig},
    AffineRepr, CurveConfig, Group,
};
use sp_ark_utils::{deserialize_result, serialize_argument};

use crate::util::{
    read_g1_compressed, read_g1_uncompressed, serialize_fq, EncodingFlags, G1_SERIALIZED_SIZE,
};
use crate::{ArkScale, HostFunctions};
use ark_bls12_381::{fr, fr::Fr, Fq};

pub type G1Affine<H> = bls12::G1Affine<crate::Config<H>>;
pub type G1Projective<H> = bls12::G1Projective<crate::Config<H>>;

#[derive(Clone, Default, PartialEq, Eq)]

pub struct Config<H: HostFunctions>(PhantomData<fn() -> H>);

impl<H: HostFunctions> CurveConfig for Config<H> {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = (x - 1)^2 / 3  = 76329603384216526031706109802092473003
    const COFACTOR: &'static [u64] = &[0x8c00aaab0000aaab, 0x396c8c005555e156];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 52435875175126190458656871551744051925719901746859129887267498875565241663483
    const COFACTOR_INV: Fr =
        MontFp!("52435875175126190458656871551744051925719901746859129887267498875565241663483");
}

impl<H: HostFunctions> SWCurveConfig for Config<H> {
    /// COEFF_A = 0
    const COEFF_A: Fq = Fq::ZERO;

    /// COEFF_B = 4
    const COEFF_B: Fq = MontFp!("4");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: G1Affine<H> = G1Affine::<H>::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }

    #[inline]
    fn is_in_correct_subgroup_assuming_on_curve(p: &G1Affine<H>) -> bool {
        // Algorithm from Section 6 of https://eprint.iacr.org/2021/1130.
        //
        // Check that endomorphism_p(P) == -[X^2]P

        // An early-out optimization described in Section 6.
        // If uP == P but P != point of infinity, then the point is not in the right
        // subgroup.
        let x_times_p = p.mul_bigint(crate::Config::<H>::X);
        if x_times_p.eq(p) && !p.infinity {
            return false;
        }

        let minus_x_squared_times_p = x_times_p.mul_bigint(crate::Config::<H>::X).neg();
        let endomorphism_p = endomorphism(p);
        minus_x_squared_times_p.eq(&endomorphism_p)
    }

    #[inline]
    fn clear_cofactor(p: &G1Affine<H>) -> G1Affine<H> {
        // Using the effective cofactor, as explained in
        // Section 5 of https://eprint.iacr.org/2019/403.pdf.
        //
        // It is enough to multiply by (1 - x), instead of (x - 1)^2 / 3
        let h_eff =
            one_minus_x(crate::Config::<H>::X_IS_NEGATIVE, crate::Config::<H>::X).into_bigint();
        Config::<H>::mul_affine(p, h_eff.as_ref()).into()
    }

    fn deserialize_with_mode<R: ark_serialize::Read>(
        mut reader: R,
        compress: ark_serialize::Compress,
        validate: ark_serialize::Validate,
    ) -> Result<Affine<Self>, ark_serialize::SerializationError> {
        let p = if compress == ark_serialize::Compress::Yes {
            read_g1_compressed(&mut reader)?
        } else {
            read_g1_uncompressed(&mut reader)?
        };

        if validate == ark_serialize::Validate::Yes && !p.is_in_correct_subgroup_assuming_on_curve()
        {
            return Err(ark_serialize::SerializationError::InvalidData);
        }
        Ok(p)
    }

    fn serialize_with_mode<W: ark_serialize::Write>(
        item: &Affine<Self>,
        mut writer: W,
        compress: ark_serialize::Compress,
    ) -> Result<(), ark_serialize::SerializationError> {
        let encoding = EncodingFlags {
            is_compressed: compress == ark_serialize::Compress::Yes,
            is_infinity: item.is_zero(),
            is_lexographically_largest: item.y > -item.y,
        };
        let mut p = *item;
        if encoding.is_infinity {
            p = G1Affine::zero();
        }
        // need to access the field struct `x` directly, otherwise we get None from xy()
        // method
        let x_bytes = serialize_fq(p.x);
        if encoding.is_compressed {
            let mut bytes: [u8; G1_SERIALIZED_SIZE] = x_bytes;

            encoding.encode_flags(&mut bytes);
            writer.write_all(&bytes)?;
        } else {
            let mut bytes = [0u8; 2 * G1_SERIALIZED_SIZE];
            bytes[0..G1_SERIALIZED_SIZE].copy_from_slice(&x_bytes[..]);
            bytes[G1_SERIALIZED_SIZE..].copy_from_slice(&serialize_fq(p.y)[..]);

            encoding.encode_flags(&mut bytes);
            writer.write_all(&bytes)?;
        };

        Ok(())
    }

    fn serialized_size(compress: ark_serialize::Compress) -> usize {
        if compress == ark_serialize::Compress::Yes {
            G1_SERIALIZED_SIZE
        } else {
            G1_SERIALIZED_SIZE * 2
        }
    }

    fn msm(
        bases: &[Affine<Self>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: ArkScale<&[SWAffine<Self>]> = bases.into();
        let scalars: ArkScale<&[<Self as CurveConfig>::ScalarField]> = scalars.into();

        let result = H::bls12_381_msm_g1(bases.encode(), scalars.encode()).unwrap();

        let result: ArkScale<SWAffine<Self>> = result.into();
        result.decode().map_err(|_| 0)
    }

    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: ArkScale<Projective<Self>> = *base.into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let result = H::bls12_381_mul_projective_g1(base.encode(), scalar.encode()).unwrap();

        let result: ArkScale<SWAffine<Self>> = result.into();
        result.decode().unwrap()
    }

    fn mul_affine(base: &Affine<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: ArkScale<&SWAffine<Self>> = *base.into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let result = H::bls12_381_mul_affine_g1(base.encode(), scalar.encode()).unwrap();

        let result: ArkScale<SWAffine<Self>> = result.into();
        result.decode().unwrap()
    }
}

fn one_minus_x(x_is_negative: bool, x_value: &'static [u64]) -> Fr {
    let x: Fr = Fr::from_sign_and_limbs(!x_is_negative, x_value);
    Fr::one() - x
}

/// G1_GENERATOR_X =
/// 3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507
pub const G1_GENERATOR_X: Fq = MontFp!("3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507");

/// G1_GENERATOR_Y =
/// 1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569
pub const G1_GENERATOR_Y: Fq = MontFp!("1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569");

/// BETA is a non-trivial cubic root of unity in fq.
pub const BETA: Fq = MontFp!("793479390729215512621379701633421447060886740281060493010456487427281649075476305620758731620350");

pub fn endomorphism<T: HostFunctions>(p: &Affine<Config<T>>) -> Affine<Config<T>> {
    // Endomorphism of the points on the curve.
    // endomorphism_p(x,y) = (BETA * x, y)
    // where BETA is a non-trivial cubic root of unity in fq.
    let mut res = *p;
    res.x *= BETA;
    res
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::HostFunctions;
    use ark_std::{rand::Rng, UniformRand};

    pub struct Host {}

    impl HostFunctions for Host {
        fn bls12_381_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
            sp_io::elliptic_curves::bls12_381_multi_miller_loop(a, b)
        }
        fn bls12_381_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
            sp_io::elliptic_curves::bls12_381_final_exponentiation(f12)
        }
        fn bls12_381_msm_g1(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()> {
            sp_io::elliptic_curves::bls12_381_msm_g1(bases, bigints)
        }
        fn bls12_381_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
            sp_io::elliptic_curves::bls12_381_mul_projective_g1(base, scalar)
        }
        fn bls12_381_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
            sp_io::elliptic_curves::bls12_381_mul_affine_g1(base, scalar)
        }
        fn bls12_381_msm_g2(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()> {
            sp_io::elliptic_curves::bls12_381_msm_g2(bases, bigints)
        }
        fn bls12_381_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
            sp_io::elliptic_curves::bls12_381_mul_projective_g2(base, scalar)
        }
        fn bls12_381_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
            sp_io::elliptic_curves::bls12_381_mul_affine_g2(base, scalar)
        }
    }

    fn sample_unchecked() -> Affine<g1::Config<Host>> {
        let mut rng = ark_std::test_rng();
        loop {
            let x = fq::rand(&mut rng);
            let greatest = rng.gen();

            if let Some(p) = Affine::get_point_from_x_unchecked(x, greatest) {
                return p;
            }
        }
    }

    #[test]
    fn test_cofactor_clearing() {
        const SAMPLES: usize = 100;
        for _ in 0..SAMPLES {
            let p: Affine<g1::Config<Host>> = sample_unchecked();
            let p = p.clear_cofactor();
            assert!(p.is_on_curve());
            assert!(p.is_in_correct_subgroup_assuming_on_curve());
        }
    }
}

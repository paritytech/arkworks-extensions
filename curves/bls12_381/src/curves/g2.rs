use ark_bls12_381::{fq2::Fq2, g2::Config as ArkConfig, Fq};
use ark_ff::{Field, MontFp, Zero};
use ark_scale::{
    ark_serialize::{Compress, SerializationError, Validate},
    hazmat::ArkScaleProjective,
    scale::{Decode, Encode},
};
use ark_std::{
    io::{Read, Write},
    marker::PhantomData,
    ops::Neg,
};
use sp_ark_models::{
    bls12,
    bls12::Bls12Config,
    short_weierstrass::{Affine, Projective, SWCurveConfig},
    AffineRepr, CurveConfig, CurveGroup, Group,
};

use crate::{
    util::{
        read_g2_compressed, read_g2_uncompressed, serialize_fq, EncodingFlags, G2_SERIALIZED_SIZE,
    },
    ArkScale, CurveHooks,
};

pub use ark_bls12_381::g2::{
    G2_GENERATOR_X, G2_GENERATOR_X_C0, G2_GENERATOR_X_C1, G2_GENERATOR_Y, G2_GENERATOR_Y_C0,
    G2_GENERATOR_Y_C1,
};

// PSI_X = 1/(u+1)^((p-1)/3)
const P_POWER_ENDOMORPHISM_COEFF_0 : Fq2 = Fq2::new(
    Fq::ZERO,
    MontFp!("4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939437")
);

// PSI_Y = 1/(u+1)^((p-1)/2)
const P_POWER_ENDOMORPHISM_COEFF_1: Fq2 = Fq2::new(
    MontFp!("2973677408986561043442465346520108879172042883009249989176415018091420807192182638567116318576472649347015917690530"),
    MontFp!("1028732146235106349975324479215795277384839936929757896155643118032610843298655225875571310552543014690878354869257")
);

// PSI_2_X = (u+1)^((1-p^2)/3)
const DOUBLE_P_POWER_ENDOMORPHISM_COEFF_0: Fq2 = Fq2::new(
    MontFp!("4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939436"),
    Fq::ZERO
);

pub type G2Affine<H> = bls12::G2Affine<crate::Config<H>>;
pub type G2Projective<H> = bls12::G2Projective<crate::Config<H>>;

pub struct Config<H: CurveHooks>(PhantomData<fn() -> H>);

impl<H: CurveHooks> CurveConfig for Config<H> {
    type BaseField = <ArkConfig as CurveConfig>::BaseField;
    type ScalarField = <ArkConfig as CurveConfig>::ScalarField;

    const COFACTOR: &'static [u64] = <ArkConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <ArkConfig as CurveConfig>::COFACTOR_INV;
}

impl<H: CurveHooks> SWCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_B;

    const GENERATOR: Affine<Self> = Affine::<Self>::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }

    // Verbatim copy of upstream implementation.
    // Can't call it directly because of different `Affine` config.
    fn is_in_correct_subgroup_assuming_on_curve(point: &Affine<Self>) -> bool {
        let mut x_times_point = point.mul_bigint(crate::Config::<H>::X);
        if crate::Config::<H>::X_IS_NEGATIVE {
            x_times_point = -x_times_point;
        }

        let p_times_point = p_power_endomorphism(point);

        x_times_point.eq(&p_times_point)
    }

    // Verbatim copy of upstream implementation.
    // Can't call it directly because of different `Affine` config.
    #[inline]
    fn clear_cofactor(p: &Affine<Self>) -> Affine<Self> {
        // Based on Section 4.1 of https://eprint.iacr.org/2017/419.pdf
        // [h(ψ)]P = [x^2 − x − 1]P + [x − 1]ψ(P) + (ψ^2)(2P)

        // x = -15132376222941642752
        // When multiplying, use -c1 instead, and then negate the result. That's much
        // more efficient, since the scalar -c1 has less limbs and a much lower Hamming
        // weight.
        let x: &'static [u64] = crate::Config::<H>::X;
        let p_projective = p.into_group();

        // [x]P
        let x_p = Config::mul_affine(p, x).neg();
        // ψ(P)
        let psi_p = p_power_endomorphism(p);
        // (ψ^2)(2P)
        let mut psi2_p2 = double_p_power_endomorphism(&p_projective.double());

        // tmp = [x]P + ψ(P)
        let mut tmp = x_p;
        tmp += &psi_p;

        // tmp2 = [x^2]P + [x]ψ(P)
        let mut tmp2: Projective<Config<H>> = tmp;
        tmp2 = tmp2.mul_bigint(x).neg();

        // add up all the terms
        psi2_p2 += tmp2;
        psi2_p2 -= x_p;
        psi2_p2 += &-psi_p;
        (psi2_p2 - p_projective).into_affine()
    }

    // Verbatim copy of upstream implementation.
    // Can't call it directly because of different `Affine` config.
    fn deserialize_with_mode<R: Read>(
        mut reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<Affine<Self>, SerializationError> {
        let p = if compress == Compress::Yes {
            read_g2_compressed(&mut reader)?
        } else {
            read_g2_uncompressed(&mut reader)?
        };

        if validate == Validate::Yes && !p.is_in_correct_subgroup_assuming_on_curve() {
            return Err(SerializationError::InvalidData);
        }
        Ok(p)
    }

    // Verbatim copy of upstream implementation.
    // Can't call it directly because of different `Affine` config.
    fn serialize_with_mode<W: Write>(
        item: &Affine<Self>,
        mut writer: W,
        compress: Compress,
    ) -> Result<(), SerializationError> {
        let encoding = EncodingFlags {
            is_compressed: compress == Compress::Yes,
            is_infinity: item.is_zero(),
            is_lexographically_largest: item.y > -item.y,
        };
        let mut p = *item;
        if encoding.is_infinity {
            p = Affine::<Self>::zero();
        }

        let mut x_bytes = [0u8; G2_SERIALIZED_SIZE];
        let c1_bytes = serialize_fq(p.x.c1);
        let c0_bytes = serialize_fq(p.x.c0);
        x_bytes[0..48].copy_from_slice(&c1_bytes[..]);
        x_bytes[48..96].copy_from_slice(&c0_bytes[..]);
        if encoding.is_compressed {
            let mut bytes: [u8; G2_SERIALIZED_SIZE] = x_bytes;

            encoding.encode_flags(&mut bytes);
            writer.write_all(&bytes)?;
        } else {
            let mut bytes = [0u8; 2 * G2_SERIALIZED_SIZE];

            let mut y_bytes = [0u8; G2_SERIALIZED_SIZE];
            let c1_bytes = serialize_fq(p.y.c1);
            let c0_bytes = serialize_fq(p.y.c0);
            y_bytes[0..48].copy_from_slice(&c1_bytes[..]);
            y_bytes[48..96].copy_from_slice(&c0_bytes[..]);
            bytes[0..G2_SERIALIZED_SIZE].copy_from_slice(&x_bytes);
            bytes[G2_SERIALIZED_SIZE..].copy_from_slice(&y_bytes);

            encoding.encode_flags(&mut bytes);
            writer.write_all(&bytes)?;
        };

        Ok(())
    }

    // Verbatim copy of upstream implementation.
    // Can't call it directly because of different `Affine` config.
    fn serialized_size(compress: Compress) -> usize {
        <ArkConfig as SWCurveConfig>::serialized_size(compress)
    }

    /// Multi scalar multiplication jumping into the user-defined `msm_g2` hook.
    ///
    /// On any internal error returns `Err(0)`.
    fn msm(
        bases: &[Affine<Self>],
        scalars: &[Self::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: ArkScale<&[Affine<Self>]> = bases.into();
        let scalars: ArkScale<&[Self::ScalarField]> = scalars.into();

        let res = H::bls12_381_msm_g2(bases.encode(), scalars.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map_err(|_| 0).map(|res| res.0)
    }

    /// Projective multiplication jumping into the user-defined `mul_projective_g2` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: ArkScaleProjective<Projective<Self>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let res =
            H::bls12_381_mul_projective_g2(base.encode(), scalar.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map(|v| v.0).unwrap_or_default()
    }

    /// Affine multiplication jumping into the user-defined `mul_projective_g2` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    fn mul_affine(base: &Affine<Self>, scalar: &[u64]) -> Projective<Self> {
        Self::mul_projective(&(*base).into(), scalar)
    }
}

/// psi(P) is the untwist-Frobenius-twist endomorhism on E'(Fq2)
fn p_power_endomorphism<H: CurveHooks>(p: &Affine<Config<H>>) -> Affine<Config<H>> {
    // The p-power endomorphism for G2 is defined as follows:
    // 1. Note that G2 is defined on curve E': y^2 = x^3 + 4(u+1).
    //    To map a point (x, y) in E' to (s, t) in E,
    //    set s = x / ((u+1) ^ (1/3)), t = y / ((u+1) ^ (1/2)),
    //    because E: y^2 = x^3 + 4.
    // 2. Apply theFrobenius endomorphism (s, t) => (s', t'),
    //    another point on curve E, where s' = s^p, t' = t^p.
    // 3. Map the point From E back to E'; that is,
    //    set x' = s' * ((u+1) ^ (1/3)), y' = t' * ((u+1) ^ (1/2)).
    //
    // To sum up, it maps
    // (x,y) -> (x^p / ((u+1)^((p-1)/3)), y^p / ((u+1)^((p-1)/2)))
    // as implemented in the code as follows.

    let mut res = *p;
    res.x.frobenius_map_in_place(1);
    res.y.frobenius_map_in_place(1);

    let tmp_x = res.x;
    res.x.c0 = -P_POWER_ENDOMORPHISM_COEFF_0.c1 * tmp_x.c1;
    res.x.c1 = P_POWER_ENDOMORPHISM_COEFF_0.c1 * tmp_x.c0;
    res.y *= P_POWER_ENDOMORPHISM_COEFF_1;

    res
}

/// For a p-power endomorphism psi(P), compute psi(psi(P))
fn double_p_power_endomorphism<H: CurveHooks>(p: &Projective<Config<H>>) -> Projective<Config<H>> {
    let mut res = *p;

    res.x *= DOUBLE_P_POWER_ENDOMORPHISM_COEFF_0;
    res.y = res.y.neg();

    res
}

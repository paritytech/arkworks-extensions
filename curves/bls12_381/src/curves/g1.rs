use crate::{
    util::{
        read_g1_compressed, read_g1_uncompressed, serialize_fq, EncodingFlags, G1_SERIALIZED_SIZE,
    },
    CurveHooks,
};

use ark_bls12_381::g1::Config as ArkConfig;
use ark_ff::{PrimeField, Zero};
use ark_models_ext::{
    bls12,
    bls12::Bls12Config,
    short_weierstrass::{Affine, SWCurveConfig},
    AffineRepr, CurveConfig, Group,
};
use ark_serialize::{Compress, SerializationError, Validate};
use ark_std::{
    io::{Read, Write},
    marker::PhantomData,
    ops::Neg,
    One,
};

pub use ark_bls12_381::g1::{BETA, G1_GENERATOR_X, G1_GENERATOR_Y};

pub type G1Affine<H> = bls12::G1Affine<crate::Config<H>>;
pub type G1Projective<H> = bls12::G1Projective<crate::Config<H>>;

#[derive(Clone, Copy)]
pub struct Config<H: CurveHooks>(PhantomData<fn() -> H>);

impl<H: CurveHooks> CurveConfig for Config<H> {
    const COFACTOR: &'static [u64] = <ArkConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <ArkConfig as CurveConfig>::COFACTOR_INV;

    type BaseField = <ArkConfig as CurveConfig>::BaseField;
    type ScalarField = <ArkConfig as CurveConfig>::ScalarField;
}

impl<H: CurveHooks> SWCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_B;

    const GENERATOR: G1Affine<H> = G1Affine::<H>::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);

    /// Multi scalar multiplication jumping into the user-defined `msm_g1` hook.
    ///
    /// On any internal error returns `Err(0)`.
    #[inline(always)]
    fn msm(bases: &[G1Affine<H>], scalars: &[Self::ScalarField]) -> Result<G1Projective<H>, usize> {
        if bases.len() != scalars.len() {
            return Err(bases.len().min(scalars.len()));
        }
        H::bls12_381_msm_g1(bases, scalars).map_err(|_| 0)
    }

    /// Projective multiplication jumping into the user-defined `mul_projective` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_projective(base: &G1Projective<H>, scalar: &[u64]) -> G1Projective<H> {
        H::bls12_381_mul_projective_g1(base, scalar).unwrap_or_default()
    }

    /// Affine multiplication jumping into the user-defined `mul_projective` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_affine(base: &G1Affine<H>, scalar: &[u64]) -> G1Projective<H> {
        Self::mul_projective(&(*base).into(), scalar)
    }

    #[inline(always)]
    fn mul_by_a(_elem: Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }

    // Verbatim copy of upstream implementation.
    //
    // Can't call it directly because of different `Affine` configuration.
    #[inline(always)]
    fn is_in_correct_subgroup_assuming_on_curve(p: &G1Affine<H>) -> bool {
        let x_times_p = p.mul_bigint(crate::Config::<H>::X);
        if x_times_p.eq(p) && !p.infinity {
            return false;
        }

        let minus_x_squared_times_p = x_times_p.mul_bigint(crate::Config::<H>::X).neg();
        let endomorphism_p = endomorphism(p);
        minus_x_squared_times_p.eq(&endomorphism_p)
    }

    // Verbatim copy of upstream implementation.
    //
    // Can't call it directly because of different `Affine` configuration.
    #[inline(always)]
    fn clear_cofactor(p: &G1Affine<H>) -> G1Affine<H> {
        let h_eff =
            one_minus_x(crate::Config::<H>::X_IS_NEGATIVE, crate::Config::<H>::X).into_bigint();
        Self::mul_affine(p, h_eff.as_ref()).into()
    }

    #[inline(always)]
    fn serialized_size(compress: Compress) -> usize {
        <ArkConfig as SWCurveConfig>::serialized_size(compress)
    }

    // Verbatim copy of upstream implementation.
    //
    // Can't call it directly because of different `Affine` configuration.
    fn serialize_with_mode<W: Write>(
        item: &G1Affine<H>,
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

    // Verbatim copy of upstream implementation.
    //
    // Can't call it directly because of different `Affine` configuration.
    fn deserialize_with_mode<R: Read>(
        mut reader: R,
        compress: Compress,
        validate: Validate,
    ) -> Result<G1Affine<H>, SerializationError> {
        let p = if compress == Compress::Yes {
            read_g1_compressed(&mut reader)?
        } else {
            read_g1_uncompressed(&mut reader)?
        };

        if validate == Validate::Yes && !p.is_in_correct_subgroup_assuming_on_curve() {
            return Err(SerializationError::InvalidData);
        }
        Ok(p)
    }
}

fn one_minus_x(
    x_is_negative: bool,
    x_value: &'static [u64],
) -> <ArkConfig as CurveConfig>::ScalarField {
    let x = <ArkConfig as CurveConfig>::ScalarField::from_sign_and_limbs(!x_is_negative, x_value);
    <ArkConfig as CurveConfig>::ScalarField::one() - x
}

pub fn endomorphism<H: CurveHooks>(p: &G1Affine<H>) -> G1Affine<H> {
    // Endomorphism of the points on the curve.
    // endomorphism_p(x,y) = (BETA * x, y)
    // where BETA is a non-trivial cubic root of unity in fq.
    let mut res = *p;
    res.x *= BETA;
    res
}

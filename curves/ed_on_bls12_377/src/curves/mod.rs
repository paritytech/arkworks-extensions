use crate::ArkScale;

use ark_ed_on_bls12_377::EdwardsConfig as ArkConfig;
use ark_ff::MontFp;
use ark_scale::{
    hazmat::ArkScaleProjective,
    scale::{Decode, Encode},
};
use ark_std::{marker::PhantomData, vec::Vec};
use sp_ark_models::{
    twisted_edwards::{Affine, MontCurveConfig, Projective, TECurveConfig},
    CurveConfig,
};

#[cfg(test)]
mod tests;

// TODO: @davxy
// Directly use upstream generator values as soon as version > 0.4.0 is released.
// Ref: https://github.com/arkworks-rs/curves/pull/150

/// GENERATOR_X =
/// 4497879464030519973909970603271755437257548612157028181994697785683032656389,
const GENERATOR_X: <ArkConfig as CurveConfig>::BaseField =
    MontFp!("4497879464030519973909970603271755437257548612157028181994697785683032656389");

/// GENERATOR_Y =
/// 4357141146396347889246900916607623952598927460421559113092863576544024487809
const GENERATOR_Y: <ArkConfig as CurveConfig>::BaseField =
    MontFp!("4357141146396347889246900916607623952598927460421559113092863576544024487809");

pub type EdwardsAffine<H> = Affine<EdwardsConfig<H>>;
pub type EdwardsProjective<H> = Projective<EdwardsConfig<H>>;

pub struct EdwardsConfig<H: CurveHooks>(PhantomData<fn() -> H>);

pub trait CurveHooks: 'static {
    fn ed_on_bls12_377_msm(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn ed_on_bls12_377_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
}

impl<H: CurveHooks> CurveConfig for EdwardsConfig<H> {
    type BaseField = <ArkConfig as CurveConfig>::BaseField;
    type ScalarField = <ArkConfig as CurveConfig>::ScalarField;

    const COFACTOR: &'static [u64] = <ArkConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <ArkConfig as CurveConfig>::COFACTOR_INV;
}

impl<H: CurveHooks> TECurveConfig for EdwardsConfig<H> {
    type MontCurveConfig = EdwardsConfig<H>;

    const COEFF_A: Self::BaseField = <ArkConfig as TECurveConfig>::COEFF_A;
    const COEFF_D: Self::BaseField = <ArkConfig as TECurveConfig>::COEFF_D;

    const GENERATOR: Affine<Self> = Affine::<Self>::new_unchecked(GENERATOR_X, GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as TECurveConfig>::mul_by_a(elem)
    }

    /// Multi scalar multiplication jumping into the user-defined `msm` hook.
    ///
    /// On any internal error returns `Err(0)`.
    fn msm(
        bases: &[Affine<Self>],
        scalars: &[Self::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: ArkScale<&[Affine<Self>]> = bases.into();
        let scalars: ArkScale<&[Self::ScalarField]> = scalars.into();

        let res = H::ed_on_bls12_377_msm(bases.encode(), scalars.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<EdwardsConfig<H>>>::decode(&mut res.as_slice());
        res.map(|res| res.0).map_err(|_| 0)
    }

    /// Projective multiplication jumping into the user-defined `mul_projective` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: ArkScaleProjective<Projective<Self>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let res =
            H::ed_on_bls12_377_mul_projective(base.encode(), scalar.encode()).unwrap_or_default();

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

impl<H: CurveHooks> MontCurveConfig for EdwardsConfig<H> {
    type TECurveConfig = EdwardsConfig<H>;

    const COEFF_A: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_B;
}

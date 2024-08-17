use crate::CurveHooks;

use ark_bw6_761::g1::Config as ArkConfig;
use ark_ff::Zero;
use ark_models_ext::{
    bw6,
    {short_weierstrass::SWCurveConfig, CurveConfig},
};
use ark_std::marker::PhantomData;

pub use ark_bw6_761::g1::{G1_GENERATOR_X, G1_GENERATOR_Y};

pub type G1Affine<H> = bw6::G1Affine<crate::Config<H>>;
pub type G1Projective<H> = bw6::G1Projective<crate::Config<H>>;

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
        H::bw6_761_msm_g1(bases, scalars).map_err(|_| 0)
    }

    /// Projective multiplication jumping into the user-defined `mul_projective_g1` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_projective(base: &G1Projective<H>, scalar: &[u64]) -> G1Projective<H> {
        H::bw6_761_mul_projective_g1(base, scalar).unwrap_or_default()
    }

    /// Affine multiplication jumping into the user-defined `mul_projective_g1` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_affine(base: &G1Affine<H>, scalar: &[u64]) -> G1Projective<H> {
        <Self as SWCurveConfig>::mul_projective(&(*base).into(), scalar)
    }

    #[inline(always)]
    fn mul_by_a(_elem: Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

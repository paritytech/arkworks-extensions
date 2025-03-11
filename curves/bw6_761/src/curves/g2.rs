use crate::CurveHooks;

use ark_bw6_761::g2::Config as ArkConfig;
use ark_models_ext::{
    bw6,
    {short_weierstrass::SWCurveConfig, CurveConfig},
};
use ark_std::marker::PhantomData;

pub use ark_bw6_761::g2::{G2_GENERATOR_X, G2_GENERATOR_Y};

pub type G2Affine<H> = bw6::G2Affine<crate::Config<H>>;
pub type G2Projective<H> = bw6::G2Projective<crate::Config<H>>;

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

    const GENERATOR: G2Affine<H> = G2Affine::<H>::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    /// Multi scalar multiplication jumping into the user-defined `msm_g2` hook.
    #[inline(always)]
    fn msm(bases: &[G2Affine<H>], scalars: &[Self::ScalarField]) -> Result<G2Projective<H>, usize> {
        if bases.len() != scalars.len() {
            return Err(bases.len().min(scalars.len()));
        }
        Ok(H::msm_g2(bases, scalars))
    }

    /// Projective multiplication jumping into the user-defined `mul_projective_g2` hook.
    #[inline(always)]
    fn mul_projective(base: &G2Projective<H>, scalar: &[u64]) -> G2Projective<H> {
        H::mul_projective_g2(base, scalar)
    }

    /// Affine multiplication jumping into the user-defined `mul_projective_g2` hook.
    #[inline(always)]
    fn mul_affine(base: &G2Affine<H>, scalar: &[u64]) -> G2Projective<H> {
        Self::mul_projective(&(*base).into(), scalar)
    }

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as SWCurveConfig>::mul_by_a(elem)
    }
}

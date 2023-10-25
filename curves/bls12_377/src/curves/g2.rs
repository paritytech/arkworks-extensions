use crate::CurveHooks;

use ark_bls12_377::g2::Config as ArkConfig;
use ark_models_ext::{
    bls12,
    short_weierstrass::{Affine, Projective, SWCurveConfig},
    CurveConfig,
};
use ark_std::marker::PhantomData;

pub use ark_bls12_377::g2::{
    G2_GENERATOR_X, G2_GENERATOR_X_C0, G2_GENERATOR_X_C1, G2_GENERATOR_Y, G2_GENERATOR_Y_C0,
    G2_GENERATOR_Y_C1,
};

pub type G2Affine<H> = bls12::G2Affine<crate::curves::Config<H>>;
pub type G2Projective<H> = bls12::G2Projective<crate::curves::Config<H>>;

#[derive(Clone, Copy)]
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

    /// Multi scalar multiplication jumping into the user-defined `msm_g2` hook.
    ///
    /// On any internal error returns `Err(0)`.
    #[inline(always)]
    fn msm(
        bases: &[Affine<Self>],
        scalars: &[Self::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        if bases.len() != scalars.len() {
            return Err(bases.len().min(scalars.len()));
        }
        H::bls12_377_msm_g2(bases, scalars).map_err(|_| 0)
    }

    /// Projective multiplication jumping into the user-defined `mul_projective_g2` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        H::bls12_377_mul_projective_g2(base, scalar).unwrap_or_default()
    }

    /// Affine multiplication jumping into the user-defined `mul_projective_g2` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_affine(base: &Affine<Self>, scalar: &[u64]) -> Projective<Self> {
        Self::mul_projective(&(*base).into(), scalar)
    }

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as SWCurveConfig>::mul_by_a(elem)
    }
}

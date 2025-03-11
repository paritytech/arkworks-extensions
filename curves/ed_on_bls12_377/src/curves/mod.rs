use ark_ed_on_bls12_377::{EdwardsConfig as ArkConfig, GENERATOR_X, GENERATOR_Y};
use ark_models_ext::{
    twisted_edwards::{self, MontCurveConfig, TECurveConfig},
    CurveConfig,
};
use ark_std::marker::PhantomData;

#[cfg(test)]
mod tests;

pub type EdwardsAffine<H> = twisted_edwards::Affine<EdwardsConfig<H>>;
pub type EdwardsProjective<H> = twisted_edwards::Projective<EdwardsConfig<H>>;

#[derive(Clone, Copy)]
pub struct EdwardsConfig<H: CurveHooks>(PhantomData<fn() -> H>);

/// Hooks for *Ed-on-BLS12-377*.
pub trait CurveHooks: 'static + Sized {
    /// Twisted Edwards multi scalar multiplication.
    fn msm(
        bases: &[EdwardsAffine<Self>],
        scalars: &[<EdwardsConfig<Self> as CurveConfig>::ScalarField],
    ) -> EdwardsProjective<Self>;

    /// Twisted Edwards projective multiplication.
    fn mul_projective(base: &EdwardsProjective<Self>, scalar: &[u64]) -> EdwardsProjective<Self>;
}

impl<H: CurveHooks> CurveConfig for EdwardsConfig<H> {
    const COFACTOR: &'static [u64] = <ArkConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <ArkConfig as CurveConfig>::COFACTOR_INV;

    type BaseField = <ArkConfig as CurveConfig>::BaseField;
    type ScalarField = <ArkConfig as CurveConfig>::ScalarField;
}

impl<H: CurveHooks> TECurveConfig for EdwardsConfig<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as TECurveConfig>::COEFF_A;
    const COEFF_D: Self::BaseField = <ArkConfig as TECurveConfig>::COEFF_D;

    const GENERATOR: EdwardsAffine<H> = EdwardsAffine::<H>::new_unchecked(GENERATOR_X, GENERATOR_Y);

    type MontCurveConfig = Self;

    /// Multi scalar multiplication jumping into the user-defined `msm` hook.
    #[inline(always)]
    fn msm(
        bases: &[EdwardsAffine<H>],
        scalars: &[Self::ScalarField],
    ) -> Result<EdwardsProjective<H>, usize> {
        if bases.len() != scalars.len() {
            return Err(bases.len().min(scalars.len()));
        }
        Ok(H::msm(bases, scalars))
    }

    /// Projective multiplication jumping into the user-defined `mul_projective` hook.
    ///
    /// On any *external* error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_projective(base: &EdwardsProjective<H>, scalar: &[u64]) -> EdwardsProjective<H> {
        H::mul_projective(base, scalar)
    }

    /// Affine multiplication jumping into the user-defined `mul_projective_g2` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_affine(base: &EdwardsAffine<H>, scalar: &[u64]) -> EdwardsProjective<H> {
        Self::mul_projective(&(*base).into(), scalar)
    }

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as TECurveConfig>::mul_by_a(elem)
    }
}

impl<H: CurveHooks> MontCurveConfig for EdwardsConfig<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_B;

    type TECurveConfig = Self;
}

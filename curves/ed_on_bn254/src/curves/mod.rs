use ark_ed_on_bn254::{EdwardsConfig as ArkConfig, GENERATOR_X, GENERATOR_Y};
use ark_models_ext::{
    transmute::{CompatibleConfig, TransmuteInto, TransmuteRef},
    twisted_edwards::{self, MontCurveConfig, TECurveConfig},
    CurveConfig, VariableBaseMSM,
};
use ark_std::marker::PhantomData;

#[cfg(test)]
mod tests;

pub type EdwardsAffine<H> = twisted_edwards::Affine<EdwardsConfig<H>>;
pub type EdwardsProjective<H> = twisted_edwards::Projective<EdwardsConfig<H>>;

#[derive(Clone, Copy)]
pub struct EdwardsConfig<H: CurveHooks>(PhantomData<fn() -> H>);

impl<H: CurveHooks> CompatibleConfig<ArkConfig> for EdwardsConfig<H> {}

/// Hooks for *Ed-on-BN254*.
///
/// All methods have default implementations that delegate to the upstream arkworks
/// operations via zero-cost transmutation.
pub trait CurveHooks: 'static + Sized {
    /// Twisted Edwards multi scalar multiplication.
    fn msm(
        bases: &[EdwardsAffine<Self>],
        scalars: &[<EdwardsConfig<Self> as CurveConfig>::ScalarField],
    ) -> EdwardsProjective<Self> {
        let bases: &[twisted_edwards::Affine<ArkConfig>] = bases.transmute_ref();
        <twisted_edwards::Projective<ArkConfig> as VariableBaseMSM>::msm_unchecked(bases, scalars)
            .transmute_into()
    }

    /// Twisted Edwards projective multiplication.
    fn mul_projective(base: &EdwardsProjective<Self>, scalar: &[u64]) -> EdwardsProjective<Self> {
        let base: &twisted_edwards::Projective<ArkConfig> = base.transmute_ref();
        <ArkConfig as TECurveConfig>::mul_projective(base, scalar).transmute_into()
    }
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
    #[inline(always)]
    fn mul_projective(base: &EdwardsProjective<H>, scalar: &[u64]) -> EdwardsProjective<H> {
        H::mul_projective(base, scalar)
    }

    /// Affine multiplication jumping into the user-defined `mul_projective` hook.
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

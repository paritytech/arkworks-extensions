use ark_models_ext::{
    models::CurveConfig,
    short_weierstrass::{self, SWCurveConfig},
};
use ark_std::marker::PhantomData;
use ark_vesta::{VestaConfig as ArkConfig, G_GENERATOR_X, G_GENERATOR_Y};

#[cfg(test)]
mod tests;

pub type Affine<H> = short_weierstrass::Affine<VestaConfig<H>>;
pub type Projective<H> = short_weierstrass::Projective<VestaConfig<H>>;

#[derive(Clone, Copy)]
pub struct VestaConfig<H: CurveHooks>(PhantomData<fn() -> H>);

/// Hooks for *Vesta*.
pub trait CurveHooks: 'static + Sized {
    /// Short Weierstrass multi scalar multiplication.
    fn msm(
        bases: &[Affine<Self>],
        scalars: &[<VestaConfig<Self> as CurveConfig>::ScalarField],
    ) -> Projective<Self>;

    /// Short Weierstrass projective multiplication.
    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self>;
}

impl<H: CurveHooks> CurveConfig for VestaConfig<H> {
    const COFACTOR: &'static [u64] = <ArkConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <ArkConfig as CurveConfig>::COFACTOR_INV;

    type BaseField = <ArkConfig as CurveConfig>::BaseField;
    type ScalarField = <ArkConfig as CurveConfig>::ScalarField;
}

impl<H: CurveHooks> SWCurveConfig for VestaConfig<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_B;

    const GENERATOR: Affine<H> = Affine::<H>::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);

    /// Multi scalar multiplication jumping into the user-defined `vesta_msm` hook.
    #[inline(always)]
    fn msm(bases: &[Affine<H>], scalars: &[Self::ScalarField]) -> Result<Projective<H>, usize> {
        if bases.len() != scalars.len() {
            return Err(bases.len().min(scalars.len()));
        }
        Ok(H::msm(bases, scalars))
    }

    /// Projective multiplication jumping into the user-defined `vesta_mul_projective` hook.
    #[inline(always)]
    fn mul_projective(base: &Projective<H>, scalar: &[u64]) -> Projective<H> {
        H::mul_projective(base, scalar)
    }
}

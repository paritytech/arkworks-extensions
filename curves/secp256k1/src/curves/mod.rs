use ark_models_ext::{
    models::CurveConfig,
    short_weierstrass::{self, SWCurveConfig},
};
use ark_secp256k1::{Config as ArkConfig, G_GENERATOR_X, G_GENERATOR_Y};
use ark_std::marker::PhantomData;

#[cfg(test)]
mod tests;

pub type Affine<H> = short_weierstrass::Affine<Secp256k1Config<H>>;
pub type Projective<H> = short_weierstrass::Projective<Secp256k1Config<H>>;

#[derive(Clone, Copy)]
pub struct Secp256k1Config<H: CurveHooks>(PhantomData<fn() -> H>);

/// Hooks for *Secp256k1*.
pub trait CurveHooks: 'static + Sized {
    /// Short Weierstrass multi scalar multiplication.
    fn msm(
        bases: &[Affine<Self>],
        scalars: &[<Secp256k1Config<Self> as CurveConfig>::ScalarField],
    ) -> Projective<Self>;

    /// Short Weierstrass projective multiplication.
    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self>;
}

impl<H: CurveHooks> CurveConfig for Secp256k1Config<H> {
    const COFACTOR: &'static [u64] = <ArkConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <ArkConfig as CurveConfig>::COFACTOR_INV;

    type BaseField = <ArkConfig as CurveConfig>::BaseField;
    type ScalarField = <ArkConfig as CurveConfig>::ScalarField;
}

impl<H: CurveHooks> SWCurveConfig for Secp256k1Config<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_B;

    const GENERATOR: Affine<H> = Affine::<H>::new_unchecked(G_GENERATOR_X, G_GENERATOR_Y);

    /// Multi scalar multiplication jumping into the user-defined `msm` hook.
    #[inline(always)]
    fn msm(bases: &[Affine<H>], scalars: &[Self::ScalarField]) -> Result<Projective<H>, usize> {
        if bases.len() != scalars.len() {
            return Err(bases.len().min(scalars.len()));
        }
        Ok(H::msm(bases, scalars))
    }

    /// Projective multiplication jumping into the user-defined `mul_projective` hook.
    fn mul_projective(base: &Projective<H>, scalar: &[u64]) -> Projective<H> {
        H::mul_projective(base, scalar)
    }
}

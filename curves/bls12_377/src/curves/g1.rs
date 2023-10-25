use crate::CurveHooks;

use ark_bls12_377::g1::Config as ArkConfig;
use ark_models_ext::{
    bls12,
    short_weierstrass::{Affine as SWAffine, Projective as SWProjective, SWCurveConfig},
    twisted_edwards::{
        Affine as TEAffine, MontCurveConfig, Projective as TEProjective, TECurveConfig,
    },
    CurveConfig,
};
use ark_std::marker::PhantomData;

pub use ark_bls12_377::g1::{G1_GENERATOR_X, G1_GENERATOR_Y, TE_GENERATOR_X, TE_GENERATOR_Y};

pub type G1Affine<H> = bls12::G1Affine<crate::Config<H>>;
pub type G1Projective<H> = bls12::G1Projective<crate::Config<H>>;

#[derive(Clone, Copy)]
pub struct Config<H: CurveHooks>(PhantomData<fn() -> H>);

impl<H: CurveHooks> CurveConfig for Config<H> {
    type BaseField = <ArkConfig as CurveConfig>::BaseField;
    type ScalarField = <ArkConfig as CurveConfig>::ScalarField;

    const COFACTOR: &'static [u64] = <ArkConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <ArkConfig as CurveConfig>::COFACTOR_INV;
}

pub type G1SWAffine<H> = SWAffine<Config<H>>;
pub type G1SWProjective<H> = SWProjective<Config<H>>;
pub type G1TEAffine<H> = TEAffine<Config<H>>;
pub type G1TEProjective<H> = TEProjective<Config<H>>;

impl<H: CurveHooks> SWCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_B;

    const GENERATOR: SWAffine<Self> =
        SWAffine::<Self>::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);

    /// Multi scalar multiplication jumping into the user-defined `msm_g1` hook.
    ///
    /// On any external error returns `Err(0)`.
    #[inline(always)]
    fn msm(
        bases: &[SWAffine<Self>],
        scalars: &[Self::ScalarField],
    ) -> Result<SWProjective<Self>, usize> {
        if bases.len() != scalars.len() {
            return Err(bases.len().min(scalars.len()));
        }
        H::bls12_377_msm_g1(bases, scalars).map_err(|_| 0)
    }

    /// Projective multiplication jumping into the user-defined `mul_projective_g1` hook.
    ///
    /// On any external error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_projective(base: &SWProjective<Self>, scalar: &[u64]) -> SWProjective<Self> {
        H::bls12_377_mul_projective_g1(base, scalar).unwrap_or_default()
    }

    /// Affine multiplication jumping into the user-defined `mul_projective_g1` hook.
    ///
    /// On any external error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_affine(base: &SWAffine<Self>, scalar: &[u64]) -> SWProjective<Self> {
        <Self as SWCurveConfig>::mul_projective(&(*base).into(), scalar)
    }

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as SWCurveConfig>::mul_by_a(elem)
    }
}

impl<H: CurveHooks> TECurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as TECurveConfig>::COEFF_A;
    const COEFF_D: Self::BaseField = <ArkConfig as TECurveConfig>::COEFF_D;

    const GENERATOR: G1TEAffine<H> = G1TEAffine::<H>::new_unchecked(TE_GENERATOR_X, TE_GENERATOR_Y);

    type MontCurveConfig = Config<H>;

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as TECurveConfig>::mul_by_a(elem)
    }
}

impl<H: CurveHooks> MontCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_B;

    type TECurveConfig = Config<H>;
}

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

    const GENERATOR: G1SWAffine<H> = G1SWAffine::<H>::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);

    /// Multi scalar multiplication jumping into the user-defined `msm_g1` hook.
    #[inline(always)]
    fn msm(
        bases: &[G1SWAffine<H>],
        scalars: &[Self::ScalarField],
    ) -> Result<G1SWProjective<H>, usize> {
        if bases.len() != scalars.len() {
            return Err(bases.len().min(scalars.len()));
        }
        Ok(H::msm_g1(bases, scalars))
    }

    /// Projective multiplication jumping into the user-defined `mul_projective_g1` hook.
    #[inline(always)]
    fn mul_projective(base: &G1SWProjective<H>, scalar: &[u64]) -> G1SWProjective<H> {
        H::mul_projective_g1(base, scalar)
    }

    /// Affine multiplication jumping into the user-defined `mul_projective_g1` hook.
    #[inline(always)]
    fn mul_affine(base: &G1SWAffine<H>, scalar: &[u64]) -> G1SWProjective<H> {
        <Self as SWCurveConfig>::mul_projective(&(*base).into(), scalar)
    }

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as SWCurveConfig>::mul_by_a(elem)
    }

    #[inline(always)]
    fn is_in_correct_subgroup_assuming_on_curve(item: &G1SWAffine<H>) -> bool {
        if Self::cofactor_is_one() {
            true
        } else {
            // Workaround for: https://github.com/arkworks-rs/algebra/issues/948
            use ark_ff::Field;
            use ark_std::Zero;
            let char = Self::ScalarField::characteristic();
            let l1 = [0, 0, char[2], char[3]];
            let l2 = [char[0], char[1], 0, 0];
            (<Self as SWCurveConfig>::mul_affine(item, &l1)
                + <Self as SWCurveConfig>::mul_affine(item, &l2))
            .is_zero()
        }
    }
}

impl<H: CurveHooks> TECurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as TECurveConfig>::COEFF_A;
    const COEFF_D: Self::BaseField = <ArkConfig as TECurveConfig>::COEFF_D;

    const GENERATOR: G1TEAffine<H> = G1TEAffine::<H>::new_unchecked(TE_GENERATOR_X, TE_GENERATOR_Y);

    type MontCurveConfig = Self;

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as TECurveConfig>::mul_by_a(elem)
    }
}

impl<H: CurveHooks> MontCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_B;

    type TECurveConfig = Self;
}

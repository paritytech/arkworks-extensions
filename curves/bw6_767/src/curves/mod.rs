use ark_bw6_767::{g1::Config as ArkG1Config, g2::Config as ArkG2Config, Config as ArkConfig};
use ark_ec::bw6::BW6Config as ArkBW6Config;
use ark_ff::PrimeField;
use ark_models_ext::{
    bw6::{BW6Config, G1Prepared, G2Prepared, TwistType, BW6},
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
    short_weierstrass::{self, SWCurveConfig},
    transmute::{TransmuteInto, TransmuteRef},
    CurveConfig, VariableBaseMSM,
};
use ark_std::marker::PhantomData;

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub use self::{
    g1::{G1Affine, G1Projective},
    g2::{G2Affine, G2Projective},
};

/// Hooks for *BW6-767* curve.
///
/// All methods have default implementations that delegate to the upstream arkworks
/// operations via zero-cost transmutation.
pub trait CurveHooks: 'static + Sized {
    /// Pairing multi Miller loop.
    fn multi_miller_loop(
        g1: impl Iterator<Item = <BW6_767<Self> as Pairing>::G1Prepared>,
        g2: impl Iterator<Item = <BW6_767<Self> as Pairing>::G2Prepared>,
    ) -> <BW6_767<Self> as Pairing>::TargetField {
        let g1 = g1.map(|p| {
            let affine: &short_weierstrass::Affine<ArkG1Config> = p.0.transmute_ref();
            *affine
        });
        let g2 = g2.map(|p| {
            let affine: &short_weierstrass::Affine<ArkG2Config> = p.0.transmute_ref();
            *affine
        });
        <ArkConfig as ArkBW6Config>::multi_miller_loop(g1, g2).0
    }

    /// Pairing final exponentiation.
    fn final_exponentiation(
        target: <BW6_767<Self> as Pairing>::TargetField,
    ) -> <BW6_767<Self> as Pairing>::TargetField {
        <ArkConfig as ArkBW6Config>::final_exponentiation(MillerLoopOutput(target))
            .map(|po| po.0)
            .unwrap_or_default()
    }

    /// Multi scalar multiplication on G1.
    fn msm_g1(
        bases: &[g1::G1Affine<Self>],
        scalars: &[<g1::Config<Self> as CurveConfig>::ScalarField],
    ) -> g1::G1Projective<Self> {
        let bases: &[short_weierstrass::Affine<ArkG1Config>] = bases.transmute_ref();
        <short_weierstrass::Projective<ArkG1Config> as VariableBaseMSM>::msm_unchecked(
            bases, scalars,
        )
        .transmute_into()
    }

    /// Multi scalar multiplication on G2.
    fn msm_g2(
        bases: &[g2::G2Affine<Self>],
        scalars: &[<g2::Config<Self> as CurveConfig>::ScalarField],
    ) -> g2::G2Projective<Self> {
        let bases: &[short_weierstrass::Affine<ArkG2Config>] = bases.transmute_ref();
        <short_weierstrass::Projective<ArkG2Config> as VariableBaseMSM>::msm_unchecked(
            bases, scalars,
        )
        .transmute_into()
    }

    /// Projective multiplication on G1.
    fn mul_projective_g1(base: &g1::G1Projective<Self>, scalar: &[u64]) -> g1::G1Projective<Self> {
        let base: &short_weierstrass::Projective<ArkG1Config> = base.transmute_ref();
        <ArkG1Config as SWCurveConfig>::mul_projective(base, scalar).transmute_into()
    }

    /// Projective multiplication on G2.
    fn mul_projective_g2(base: &g2::G2Projective<Self>, scalar: &[u64]) -> g2::G2Projective<Self> {
        let base: &short_weierstrass::Projective<ArkG2Config> = base.transmute_ref();
        <ArkG2Config as SWCurveConfig>::mul_projective(base, scalar).transmute_into()
    }
}

#[derive(Clone, Copy)]
pub struct Config<H: CurveHooks>(PhantomData<fn() -> H>);

pub type BW6_767<H> = BW6<Config<H>>;

impl<H: CurveHooks> BW6Config for Config<H> {
    const X: <Self::Fp as PrimeField>::BigInt = <ArkConfig as ArkBW6Config>::X;
    const X_IS_NEGATIVE: bool = <ArkConfig as ArkBW6Config>::X_IS_NEGATIVE;
    const TWIST_TYPE: TwistType = <ArkConfig as ArkBW6Config>::TWIST_TYPE;

    const ATE_LOOP_COUNT_1: &'static [u64] = <ArkConfig as ArkBW6Config>::ATE_LOOP_COUNT_1;
    const ATE_LOOP_COUNT_1_IS_NEGATIVE: bool =
        <ArkConfig as ArkBW6Config>::ATE_LOOP_COUNT_1_IS_NEGATIVE;

    const ATE_LOOP_COUNT_2: &'static [i8] = <ArkConfig as ArkBW6Config>::ATE_LOOP_COUNT_2;
    const ATE_LOOP_COUNT_2_IS_NEGATIVE: bool =
        <ArkConfig as ArkBW6Config>::ATE_LOOP_COUNT_2_IS_NEGATIVE;

    type Fp = <ArkConfig as ArkBW6Config>::Fp;
    type Fp3Config = <ArkConfig as ArkBW6Config>::Fp3Config;
    type Fp6Config = <ArkConfig as ArkBW6Config>::Fp6Config;

    type G1Config = g1::Config<H>;
    type G2Config = g2::Config<H>;

    /// Multi Miller loop jumping into the user-defined `multi_miller_loop` hook.
    ///
    /// For any internal error returns `TargetField::zero()`.
    #[inline(always)]
    fn multi_miller_loop(
        g1: impl IntoIterator<Item = impl Into<G1Prepared<Self>>>,
        g2: impl IntoIterator<Item = impl Into<G2Prepared<Self>>>,
    ) -> MillerLoopOutput<BW6<Self>> {
        let g1 = g1.into_iter().map(|item| item.into());
        let g2 = g2.into_iter().map(|item| item.into());
        let res = H::multi_miller_loop(g1, g2);
        MillerLoopOutput(res)
    }

    /// Final exponentiation jumping into the user-defined `final_exponentiation` hook.
    ///
    /// For any internal error returns `None`.
    #[inline(always)]
    fn final_exponentiation(
        target: MillerLoopOutput<BW6<Self>>,
    ) -> Option<PairingOutput<BW6<Self>>> {
        let res = H::final_exponentiation(target.0);
        Some(PairingOutput(res))
    }
}

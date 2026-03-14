use ark_bls12_377::{g1::Config as ArkG1Config, g2::Config as ArkG2Config, Config as ArkConfig};
use ark_ec::bls12::Bls12Config as ArkBls12Config;
use ark_models_ext::{
    bls12::{Bls12, Bls12Config, G1Prepared, G2Prepared, TwistType},
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

/// Hooks for *BLS12-377* curve.
///
/// All methods have default implementations that delegate to the upstream arkworks
/// operations via zero-cost transmutation.
pub trait CurveHooks: 'static + Sized {
    /// Pairing multi Miller loop.
    fn multi_miller_loop(
        g1: impl Iterator<Item = <Bls12_377<Self> as Pairing>::G1Prepared>,
        g2: impl Iterator<Item = <Bls12_377<Self> as Pairing>::G2Prepared>,
    ) -> <Bls12_377<Self> as Pairing>::TargetField {
        let g1 = g1.map(|p| {
            let affine: &short_weierstrass::Affine<ArkG1Config> = p.0.transmute_ref();
            *affine
        });
        let g2 = g2.map(|p| {
            let affine: &short_weierstrass::Affine<ArkG2Config> = p.0.transmute_ref();
            *affine
        });
        <ArkConfig as ArkBls12Config>::multi_miller_loop(g1, g2).0
    }

    /// Pairing final exponentiation.
    fn final_exponentiation(
        target: <Bls12_377<Self> as Pairing>::TargetField,
    ) -> <Bls12_377<Self> as Pairing>::TargetField {
        <ArkConfig as ArkBls12Config>::final_exponentiation(MillerLoopOutput(target))
            .map(|po| po.0)
            .unwrap_or_default()
    }

    /// Multi scalar multiplication on G1.
    fn msm_g1(
        bases: &[g1::G1Affine<Self>],
        scalars: &[<g1::Config<Self> as CurveConfig>::ScalarField],
    ) -> G1Projective<Self> {
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
    ) -> G2Projective<Self> {
        let bases: &[short_weierstrass::Affine<ArkG2Config>] = bases.transmute_ref();
        <short_weierstrass::Projective<ArkG2Config> as VariableBaseMSM>::msm_unchecked(
            bases, scalars,
        )
        .transmute_into()
    }

    /// Projective multiplication on G1.
    fn mul_projective_g1(base: &G1Projective<Self>, scalar: &[u64]) -> G1Projective<Self> {
        let base: &short_weierstrass::Projective<ArkG1Config> = base.transmute_ref();
        <ArkG1Config as SWCurveConfig>::mul_projective(base, scalar).transmute_into()
    }

    /// Projective multiplication on G2.
    fn mul_projective_g2(base: &G2Projective<Self>, scalar: &[u64]) -> G2Projective<Self> {
        let base: &short_weierstrass::Projective<ArkG2Config> = base.transmute_ref();
        <ArkG2Config as SWCurveConfig>::mul_projective(base, scalar).transmute_into()
    }
}

#[derive(Clone, Copy)]
pub struct Config<H: CurveHooks>(PhantomData<fn() -> H>);

pub type Bls12_377<H> = Bls12<Config<H>>;

impl<H: CurveHooks> Bls12Config for Config<H> {
    const X: &'static [u64] = <ArkConfig as ArkBls12Config>::X;
    const X_IS_NEGATIVE: bool = <ArkConfig as ArkBls12Config>::X_IS_NEGATIVE;
    const TWIST_TYPE: TwistType = <ArkConfig as ArkBls12Config>::TWIST_TYPE;

    type Fp = <ArkConfig as ArkBls12Config>::Fp;
    type Fp2Config = <ArkConfig as ArkBls12Config>::Fp2Config;
    type Fp6Config = <ArkConfig as ArkBls12Config>::Fp6Config;
    type Fp12Config = <ArkConfig as ArkBls12Config>::Fp12Config;

    type G1Config = g1::Config<H>;
    type G2Config = g2::Config<H>;

    /// Multi Miller loop jumping into the user-defined `multi_miller_loop` hook.
    #[inline(always)]
    fn multi_miller_loop(
        g1: impl IntoIterator<Item = impl Into<G1Prepared<Self>>>,
        g2: impl IntoIterator<Item = impl Into<G2Prepared<Self>>>,
    ) -> MillerLoopOutput<Bls12<Self>> {
        let g1 = g1.into_iter().map(|item| item.into());
        let g2 = g2.into_iter().map(|item| item.into());
        let res = H::multi_miller_loop(g1, g2);
        MillerLoopOutput(res)
    }

    /// Final exponentiation jumping into the user-defined `final_exponentiation` hook.
    #[inline(always)]
    fn final_exponentiation(
        target: MillerLoopOutput<Bls12<Self>>,
    ) -> Option<PairingOutput<Bls12<Self>>> {
        let res = H::final_exponentiation(target.0);
        Some(PairingOutput(res))
    }
}

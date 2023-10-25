use ark_bls12_377::Config as ArkConfig;
use ark_ec::bls12::Bls12Config as ArkBls12Config;
use ark_models_ext::{
    bls12::{Bls12, Bls12Config, G1Prepared, G2Prepared, TwistType},
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
    CurveConfig,
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
pub trait CurveHooks: 'static + Sized {
    /// Pairing multi Miller loop.
    fn bls12_377_multi_miller_loop(
        g1: impl Iterator<Item = <Bls12_377<Self> as Pairing>::G1Prepared>,
        g2: impl Iterator<Item = <Bls12_377<Self> as Pairing>::G2Prepared>,
    ) -> Result<<Bls12_377<Self> as Pairing>::TargetField, ()>;

    /// Pairing final exponentiation.
    fn bls12_377_final_exponentiation(
        target: <Bls12_377<Self> as Pairing>::TargetField,
    ) -> Result<<Bls12_377<Self> as Pairing>::TargetField, ()>;

    /// Multi scalar multiplication on G1.
    fn bls12_377_msm_g1(
        bases: &[g1::G1Affine<Self>],
        scalars: &[<g1::Config<Self> as CurveConfig>::ScalarField],
    ) -> Result<g1::G1Projective<Self>, ()>;

    /// Multi scalar multiplication on G2.
    fn bls12_377_msm_g2(
        bases: &[g2::G2Affine<Self>],
        scalars: &[<g2::Config<Self> as CurveConfig>::ScalarField],
    ) -> Result<g2::G2Projective<Self>, ()>;

    /// Projective multiplication on G1.
    fn bls12_377_mul_projective_g1(
        base: &g1::G1Projective<Self>,
        scalar: &[u64],
    ) -> Result<g1::G1Projective<Self>, ()>;

    /// Projective multiplication on G2.
    fn bls12_377_mul_projective_g2(
        base: &g2::G2Projective<Self>,
        scalar: &[u64],
    ) -> Result<g2::G2Projective<Self>, ()>;
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
    ///
    /// For any external error returns `MillerLoopOutput(TargetField::zero())`.
    #[inline(always)]
    fn multi_miller_loop(
        g1: impl IntoIterator<Item = impl Into<G1Prepared<Self>>>,
        g2: impl IntoIterator<Item = impl Into<G2Prepared<Self>>>,
    ) -> MillerLoopOutput<Bls12<Self>> {
        let g1 = g1.into_iter().map(|item| item.into());
        let g2 = g2.into_iter().map(|item| item.into());
        let res = H::bls12_377_multi_miller_loop(g1, g2);
        MillerLoopOutput(res.unwrap_or_default())
    }

    /// Final exponentiation jumping into the user-defined `final_exponentiation` hook.
    ///
    /// For any external error returns `None`.
    #[inline(always)]
    fn final_exponentiation(
        target: MillerLoopOutput<Bls12<Self>>,
    ) -> Option<PairingOutput<Bls12<Self>>> {
        let res = H::bls12_377_final_exponentiation(target.0);
        res.map(PairingOutput).ok()
    }
}

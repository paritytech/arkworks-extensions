use crate::ArkScale;

use ark_bw6_761::Config as ArkConfig;
use ark_ec::bw6::BW6Config as ArkBW6Config;
use ark_ff::PrimeField;
use ark_scale::scale::{Decode, Encode};
use ark_std::{marker::PhantomData, vec::Vec};
use sp_ark_models::{
    bw6::{BW6Config, G1Prepared, G2Prepared, TwistType, BW6},
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
};

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub use self::{
    g1::{G1Affine, G1Projective},
    g2::{G2Affine, G2Projective},
};

pub struct Config<H: CurveHooks>(PhantomData<fn() -> H>);

pub trait CurveHooks: 'static {
    fn bw6_761_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bw6_761_final_exponentiation(f12: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bw6_761_msm_g1(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bw6_761_msm_g2(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bw6_761_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bw6_761_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
}

impl<H: CurveHooks> BW6Config for Config<H> {
    type Fp = <ArkConfig as ArkBW6Config>::Fp;
    type Fp3Config = <ArkConfig as ArkBW6Config>::Fp3Config;
    type Fp6Config = <ArkConfig as ArkBW6Config>::Fp6Config;

    type G1Config = g1::Config<H>;
    type G2Config = g2::Config<H>;

    const X: <Self::Fp as PrimeField>::BigInt = <ArkConfig as ArkBW6Config>::X;
    const X_IS_NEGATIVE: bool = <ArkConfig as ArkBW6Config>::X_IS_NEGATIVE;

    const ATE_LOOP_COUNT_1: &'static [u64] = <ArkConfig as ArkBW6Config>::ATE_LOOP_COUNT_1;
    const ATE_LOOP_COUNT_1_IS_NEGATIVE: bool =
        <ArkConfig as ArkBW6Config>::ATE_LOOP_COUNT_1_IS_NEGATIVE;

    const ATE_LOOP_COUNT_2: &'static [i8] = <ArkConfig as ArkBW6Config>::ATE_LOOP_COUNT_2;
    const ATE_LOOP_COUNT_2_IS_NEGATIVE: bool =
        <ArkConfig as ArkBW6Config>::ATE_LOOP_COUNT_2_IS_NEGATIVE;

    const TWIST_TYPE: TwistType = <ArkConfig as ArkBW6Config>::TWIST_TYPE;

    /// Multi Miller loop jumping into the user-defined `multi_miller_loop` hook.
    ///
    /// For any internal error returns `TargetField::zero()`.
    fn multi_miller_loop(
        a: impl IntoIterator<Item = impl Into<G1Prepared<Self>>>,
        b: impl IntoIterator<Item = impl Into<G2Prepared<Self>>>,
    ) -> MillerLoopOutput<BW6<Self>> {
        let a: ArkScale<Vec<<BW6<Self> as Pairing>::G1Prepared>> = a
            .into_iter()
            .map(|el| {
                let el: <BW6<Self> as Pairing>::G1Prepared = el.into();
                el
            })
            .collect::<Vec<_>>()
            .into();
        let b: ArkScale<Vec<<BW6<Self> as Pairing>::G2Prepared>> = b
            .into_iter()
            .map(|el| {
                let el: <BW6<Self> as Pairing>::G2Prepared = el.into();
                el
            })
            .collect::<Vec<_>>()
            .into();

        let res = H::bw6_761_multi_miller_loop(a.encode(), b.encode()).unwrap_or_default();

        let res = ArkScale::<<BW6<Self> as Pairing>::TargetField>::decode(&mut res.as_slice());
        MillerLoopOutput(res.map(|v| v.0).unwrap_or_default())
    }

    /// Final exponentiation jumping into the user-defined `final_exponentiation` hook.
    ///
    /// For any internal error returns `None`.
    fn final_exponentiation(f: MillerLoopOutput<BW6<Self>>) -> Option<PairingOutput<BW6<Self>>> {
        let target: ArkScale<<BW6<Self> as Pairing>::TargetField> = f.0.into();

        let res = H::bw6_761_final_exponentiation(target.encode()).unwrap_or_default();

        let res = ArkScale::<PairingOutput<BW6<Self>>>::decode(&mut res.as_slice());
        res.map(|res| res.0).ok()
    }
}

pub type BW6_761<H> = BW6<Config<H>>;

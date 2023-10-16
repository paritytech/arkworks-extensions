use crate::ArkScale;

use ark_bls12_377::Config as ArkConfig;
use ark_ec::bls12::Bls12Config as ArkBls12Config;
use ark_scale::scale::{Decode, Encode};
use ark_std::{marker::PhantomData, vec::Vec};
use sp_ark_models::{
    bls12::{Bls12, Bls12Config, G1Prepared, G2Prepared, TwistType},
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

#[derive(Clone, Copy)]
pub struct Config<H: CurveHooks>(PhantomData<fn() -> H>);

pub trait CurveHooks: 'static {
    fn bls12_377_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_377_final_exponentiation(f12: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_377_msm_g1(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_377_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_377_msm_g2(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_377_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
}

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
    /// For any internal error returns `TargetField::zero()`.
    fn multi_miller_loop(
        a: impl IntoIterator<Item = impl Into<G1Prepared<Self>>>,
        b: impl IntoIterator<Item = impl Into<G2Prepared<Self>>>,
    ) -> MillerLoopOutput<Bls12<Self>> {
        let a: ArkScale<Vec<<Bls12<Self> as Pairing>::G1Prepared>> = a
            .into_iter()
            .map(|el| {
                let el: <Bls12<Self> as Pairing>::G1Prepared = el.into();
                el
            })
            .collect::<Vec<_>>()
            .into();
        let b: ArkScale<Vec<<Bls12<Self> as Pairing>::G2Prepared>> = b
            .into_iter()
            .map(|el| {
                let el: <Bls12<Self> as Pairing>::G2Prepared = el.into();
                el
            })
            .collect::<Vec<_>>()
            .into();

        let res = H::bls12_377_multi_miller_loop(a.encode(), b.encode()).unwrap_or_default();

        let res = ArkScale::<<Bls12<Self> as Pairing>::TargetField>::decode(&mut res.as_slice());
        MillerLoopOutput(res.map(|v| v.0).unwrap_or_default())
    }

    /// Final exponentiation jumping into the user-defined `final_exponentiation` hook.
    ///
    /// For any internal error returns `None`.
    fn final_exponentiation(
        f: MillerLoopOutput<Bls12<Self>>,
    ) -> Option<PairingOutput<Bls12<Self>>> {
        let target: ArkScale<<Bls12<Self> as Pairing>::TargetField> = f.0.into();

        let res = H::bls12_377_final_exponentiation(target.encode()).unwrap_or_default();

        let res = ArkScale::<PairingOutput<Bls12<Self>>>::decode(&mut res.as_slice());
        res.map(|res| res.0).ok()
    }
}

pub type Bls12_377<H> = Bls12<Config<H>>;

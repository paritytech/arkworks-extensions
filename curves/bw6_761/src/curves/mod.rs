use crate::{Fq, Fq3Config, Fq6Config};
use ark_ff::{biginteger::BigInteger768 as BigInteger, BigInt};
use ark_scale::ArkScale;
use ark_std::{marker::PhantomData, vec::Vec};
use codec::{Decode, Encode};
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

#[derive(PartialEq, Eq)]
pub struct Config<H: HostFunctions>(PhantomData<fn() -> H>);

pub trait HostFunctions: 'static {
    fn bw6_761_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bw6_761_final_exponentiation(f12: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bw6_761_msm_g1(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bw6_761_msm_g2(bases: Vec<u8>, bigints: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bw6_761_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bw6_761_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
}

impl<H: HostFunctions> BW6Config for Config<H> {
    const X: BigInteger = BigInt::new([
        0x8508c00000000001,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
        0x0,
    ]);
    /// `x` is positive.
    const X_IS_NEGATIVE: bool = false;
    // X+1
    const ATE_LOOP_COUNT_1: &'static [u64] = &[0x8508c00000000002];
    const ATE_LOOP_COUNT_1_IS_NEGATIVE: bool = false;
    // X^3-X^2-X
    const ATE_LOOP_COUNT_2: &'static [i8] = &[
        -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 1, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, 0, -1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
        0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, -1, 0, 0, 0, 0, -1, 0, 0,
        1, 0, 0, 0, -1, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, 1, 0, 0, 1, 0, -1, 0, 1, 0, 1, 0, 0, 0, 1,
        0, -1, 0, -1, 0, 0, 0, 0, 0, 1, 0, 0, 1,
    ];
    const ATE_LOOP_COUNT_2_IS_NEGATIVE: bool = false;
    const TWIST_TYPE: TwistType = TwistType::M;
    type Fp = Fq;
    type Fp3Config = Fq3Config;
    type Fp6Config = Fq6Config;
    type G1Config = g1::Config<H>;
    type G2Config = g2::Config<H>;

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
        let result = H::bw6_761_multi_miller_loop(a.encode(), b.encode()).unwrap();

        let result = <ArkScale<<BW6<Self> as Pairing>::TargetField> as Decode>::decode(
            &mut result.as_slice(),
        )
        .unwrap()
        .0;
        MillerLoopOutput(result)
    }

    fn final_exponentiation(f: MillerLoopOutput<BW6<Self>>) -> Option<PairingOutput<BW6<Self>>> {
        let target: ArkScale<<BW6<Self> as Pairing>::TargetField> = f.0.into();

        let result = H::bw6_761_final_exponentiation(target.encode());

        result.ok().map(|res| {
            <ArkScale<PairingOutput<BW6<Self>>> as Decode>::decode(&mut res.as_slice())
                .unwrap()
                .0
        })
    }
}

pub type BW6_761<H> = BW6<Config<H>>;

use crate::*;
use ark_ff::Fp12;
use ark_std::{io::Cursor, marker::PhantomData, vec::Vec};
use codec::{Decode, Encode};
use codec::{Decode, Encode};
use sp_ark_models::{
    bls12::{Bls12, Bls12Config, G1Prepared, G2Prepared, TwistType},
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
};
use sp_ark_utils::{deserialize_result, serialize_argument};

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub use self::{
    g1::{G1Affine, G1Projective},
    g2::{G2Affine, G2Projective},
};

const HOST_CALL: ark_scale::Usage = ark_scale::HOST_CALL;
pub type ArkScale<T> = ark_scale::ArkScale<T, HOST_CALL>;

pub struct Config<H: HostFunctions>(PhantomData<fn() -> H>);

pub trait HostFunctions: 'static {
    fn bls12_377_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_377_final_exponentiation(f12: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_377_msm_g1(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_377_msm_g2(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_377_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_377_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_377_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_377_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
}

impl<H: HostFunctions> Bls12Config for Config<H> {
    const X: &'static [u64] = &[0x8508c00000000001];
    /// `x` is positive.
    const X_IS_NEGATIVE: bool = false;
    const TWIST_TYPE: TwistType = TwistType::D;
    type Fp = Fq;
    type Fp2Config = Fq2Config;
    type Fp6Config = Fq6Config;
    type Fp12Config = Fq12Config;
    type G1Config = g1::Config<H>;
    type G2Config = g2::Config<H>;

    fn multi_miller_loop(
        a: impl IntoIterator<Item = impl Into<G1Prepared<Self>>>,
        b: impl IntoIterator<Item = impl Into<G2Prepared<Self>>>,
    ) -> MillerLoopOutput<Bls12<Self>> {
        let a: ArkScale<Vec<<Curve as Pairing>::G1Affine>> = a
            .into_iter()
            .map(|elem| {
                let elem: <Bls12<Self> as Pairing>::G1Prepared = elem.into();
            })
            .collect::<Vec<_>>()
            .into();
        let b: ArkScale<Vec<<Curve as Pairing>::G2Affine>> = b
            .into_iter()
            .map(|elem| {
                let elem: <Bls12<Self> as Pairing>::G2Prepared = elem.into();
            })
            .collect::<Vec<_>>()
            .into();

        let result = H::bls12_377_multi_miller_loop(a.encode(), b.encode()).unwrap();

        let result = <ArkScale<Bls12<Self>> as Decode>::decode(&mut result.clone().as_slice())
            .unwrap()
            .0;
        MillerLoopOutput(result)
    }

    fn final_exponentiation(
        f: MillerLoopOutput<Bls12<Self>>,
    ) -> Option<PairingOutput<Bls12<Self>>> {
        let target: ArkScale<Bls12<Self>> = f.0.into();

        let result = H::bls12_377_final_exponentiation(target.encode());

        let result = <ArkScale<PairingOutput<Bls12<Self>>> as Decode>::decode(
            &mut result.clone().as_slice(),
        );
        result.ok()
    }
}

pub type Bls12_377<H> = Bls12<Config<H>>;

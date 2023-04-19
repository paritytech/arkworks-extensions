use crate::*;
use ark_ff::Fp12;
use ark_std::{io::Cursor, marker::PhantomData, vec, vec::Vec};
use codec::{Decode, Encode};
use sp_ark_models::{
    bls12::{Bls12, Bls12Config, G1Prepared, G2Prepared, TwistType},
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
};
use sp_ark_utils::{deserialize_result, serialize_argument};

pub mod g1;
pub mod g2;
pub(crate) mod util;

use crate::fq;
use ark_bls12_381::{fq::Fq, fq12, fq2, fq6};

const HOST_CALL: ark_scale::Usage = ark_scale::HOST_CALL;
pub type ArkScale<T> = ark_scale::ArkScale<T, HOST_CALL>;

#[cfg(test)]
mod tests;

pub use self::{
    g1::{G1Affine, G1Projective},
    g2::{G2Affine, G2Projective},
};

pub struct Config<H: HostFunctions>(PhantomData<fn() -> H>);

pub trait HostFunctions: 'static {
    fn bls12_381_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_381_final_exponentiation(f12: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_381_msm_g1(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_381_msm_g2(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_381_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_381_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_381_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn bls12_381_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
}

impl<H: HostFunctions> Bls12Config for Config<H> {
    const X: &'static [u64] = &[0xd201000000010000];
    const X_IS_NEGATIVE: bool = true;
    const TWIST_TYPE: TwistType = TwistType::M;
    type Fp = Fq;
    type Fp2Config = fq2::Fq2Config;
    type Fp6Config = fq6::Fq6Config;
    type Fp12Config = fq12::Fq12Config;
    type G1Config = self::g1::Config<H>;
    type G2Config = self::g2::Config<H>;

    fn multi_miller_loop(
        a: impl IntoIterator<Item = impl Into<G1Prepared<Self>>>,
        b: impl IntoIterator<Item = impl Into<G2Prepared<Self>>>,
    ) -> MillerLoopOutput<Bls12<Self>> {
        let a: ArkScale<Vec<<Curve as Pairing>::G1Affine>> = a
            .into_iter()
            .map(|elem| {
                let elem: <Bls12<Self> as Pairing>::G1Prepared = elem.into();
            })
            .collect()
            .into();
        let b: ArkScale<Vec<<Curve as Pairing>::G2Affine>> = b
            .into_iter()
            .map(|elem| {
                let elem: <Bls12<Self> as Pairing>::G2Prepared = elem.into();
            })
            .collect()
            .into();

        let result = H::bls12_381_multi_miller_loop(a.encode(), b.encode()).unwrap();

        let result =
            <ArkScale<Fp12<Self::Fp12Config>> as Decode>::decode(&mut result.clone().as_slice());
        MillerLoopOutput(result.unwrap().0)
    }

    fn final_exponentiation(
        f: MillerLoopOutput<Bls12<Self>>,
    ) -> Option<PairingOutput<Bls12<Self>>> {
        let target: ArkScale<MillerLoopOutput<Bls12<Self>>> = f.0.into();

        let result = H::bls12_381_final_exponentiation(target.encode()).unwrap();

        let result =
            <ArkScale<PairingOutput<Bls12<Self>>> as Decode>::decode(&result.clone().as_slice());

        result.ok().map(|res| res.0)
    }
}

pub type Bls12_381<H> = Bls12<Config<H>>;

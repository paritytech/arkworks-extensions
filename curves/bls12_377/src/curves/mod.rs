use crate::*;
use ark_ff::Fp12;
use ark_std::{io::Cursor, marker::PhantomData, vec::Vec};
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

pub struct Config<H: HostFunctions>(PhantomData<fn() -> H>);

pub trait HostFunctions: 'static {
    fn bls12_377_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8>;
    fn bls12_377_final_exponentiation(f12: Vec<u8>) -> Vec<u8>;
    fn bls12_377_msm_g1(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8>;
    fn bls12_377_msm_g2(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8>;
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
        let a: Vec<Vec<u8>> = a
            .into_iter()
            .map(|elem| {
                let elem: <Bls12<Self> as Pairing>::G1Prepared = elem.into();
                serialize_argument(elem)
            })
            .collect();
        let b = b
            .into_iter()
            .map(|elem| {
                let elem: <Bls12<Self> as Pairing>::G2Prepared = elem.into();
                serialize_argument(elem)
            })
            .collect();

        let result = H::bls12_377_multi_miller_loop(a, b);

        let f = deserialize_result::<Fp12<Self::Fp12Config>>(&result);
        MillerLoopOutput(f)
    }

    fn final_exponentiation(
        f: MillerLoopOutput<Bls12<Self>>,
    ) -> Option<PairingOutput<Bls12<Self>>> {
        let target = serialize_argument(f.0);

        let result = H::bls12_377_final_exponentiation(target);

        let result = deserialize_result::<PairingOutput<Bls12<Self>>>(&result);
        Some(result)
    }
}

pub type Bls12_377<H> = Bls12<Config<H>>;

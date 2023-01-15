use ark_ff::{biginteger::BigInteger768 as BigInteger, BigInt};
use ark_models::{
    bw6::{BW6Config, G1Prepared, G2Prepared, TwistType, BW6},
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use ark_std::{io::Cursor, marker::PhantomData, vec, vec::Vec};

use crate::*;

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq)]
pub struct Config<H: HostFunctions>(PhantomData<fn() -> H>);

pub trait HostFunctions: 'static {
    fn bw6_761_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8>;
    fn bw6_761_final_exponentiation(f12: Vec<u8>) -> Vec<u8>;
    fn bw6_761_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bw6_761_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bw6_761_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bw6_761_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bw6_761_msm_g1(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8>;
    fn bw6_761_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8>;
}

impl<H: HostFunctions + core::cmp::Eq> BW6Config for Config<H> {
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
        let a: Vec<Vec<u8>> = a
            .into_iter()
            .map(|elem| {
                let elem: <BW6<Self> as Pairing>::G1Prepared = elem.into();
                let mut serialized = vec![0; elem.serialized_size(Compress::Yes)];
                let mut cursor = Cursor::new(&mut serialized[..]);
                elem.serialize_with_mode(&mut cursor, Compress::Yes)
                    .unwrap();
                serialized
            })
            .collect();
        let b = b
            .into_iter()
            .map(|elem| {
                let elem: <BW6<Self> as Pairing>::G2Prepared = elem.into();
                let mut serialized = vec![0u8; elem.serialized_size(Compress::Yes)];
                let mut cursor = Cursor::new(&mut serialized[..]);
                elem.serialize_with_mode(&mut cursor, Compress::Yes)
                    .unwrap();
                serialized
            })
            .collect();

        let result = H::bw6_761_multi_miller_loop(a, b);

        let cursor = Cursor::new(&result[..]);
        let f = <BW6<Self> as Pairing>::TargetField::deserialize_with_mode(
            cursor,
            Compress::Yes,
            Validate::No,
        )
        .unwrap();
        MillerLoopOutput(f)
    }

    fn final_exponentiation(f: MillerLoopOutput<BW6<Self>>) -> Option<PairingOutput<BW6<Self>>> {
        let target = f.0;
        let mut serialized_target = vec![0; target.serialized_size(Compress::Yes)];
        let mut cursor = Cursor::new(&mut serialized_target[..]);
        target
            .serialize_with_mode(&mut cursor, Compress::Yes)
            .unwrap();

        let result = H::bw6_761_final_exponentiation(serialized_target);

        let cursor = Cursor::new(&result[..]);
        let result =
            PairingOutput::<BW6<Self>>::deserialize_with_mode(cursor, Compress::Yes, Validate::No)
                .unwrap();

        Some(result)
    }
}

pub type BW6_761<H> = BW6<Config<H>>;

pub type G1Affine<H> = g1::G1Affine<Config<H>>;
pub type G1Projective<H> = g1::G1Projective<Config<H>>;
pub type G2Affine<H> = g2::G2Affine<Config<H>>;
pub type G2Projective<H> = g2::G2Projective<Config<H>>;

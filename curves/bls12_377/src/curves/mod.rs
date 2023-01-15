use ark_ff::Fp12;
use ark_models::{
    bls12,
    bls12::{Bls12, Bls12Config, G1Prepared, G2Prepared, TwistType},
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use ark_std::{io::Cursor, marker::PhantomData, vec, vec::Vec};

use crate::*;

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub struct Config<H: HostFunctions>(PhantomData<fn() -> H>);

pub trait HostFunctions: 'static {
    fn bls12_377_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8>;
    fn bls12_377_final_exponentiation(f12: Vec<u8>) -> Vec<u8>;
    fn bls12_377_msm_g1(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8>;
    fn bls12_377_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bls12_377_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bls12_377_msm_g2(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8>;
    fn bls12_377_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bls12_377_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
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
                let elem: <Bls12<Self> as Pairing>::G2Prepared = elem.into();
                let mut serialized = vec![0u8; elem.serialized_size(Compress::Yes)];
                let mut cursor = Cursor::new(&mut serialized[..]);
                elem.serialize_with_mode(&mut cursor, Compress::Yes)
                    .unwrap();
                serialized
            })
            .collect();

        let resuslt = H::bls12_377_multi_miller_loop(a, b);

        let cursor = Cursor::new(&result[..]);
        let f: <Bls12<Self> as Pairing>::TargetField =
            Fp12::deserialize_with_mode(cursor, Compress::Yes, Validate::No).unwrap();
        MillerLoopOutput(f)
    }

    fn final_exponentiation(
        f: MillerLoopOutput<Bls12<Self>>,
    ) -> Option<PairingOutput<Bls12<Self>>> {
        let target = f.0;
        let mut serialized_target = vec![0; target.serialized_size(Compress::Yes)];
        let mut cursor = Cursor::new(&mut serialized_target[..]);
        target
            .serialize_with_mode(&mut cursor, Compress::Yes)
            .unwrap();

        let result = H::bls12_377_final_exponentiation(serialized_target);

        let cursor = Cursor::new(&result[..]);
        let result = PairingOutput::<Bls12<Self>>::deserialize_with_mode(
            cursor,
            Compress::Yes,
            Validate::No,
        )
        .unwrap();

        Some(res)
    }
}

pub type Bls12_377<H> = Bls12<Config<H>>;

pub type G1Affine<H> = bls12::G1Affine<Config<H>>;
pub type G1Projective<H> = bls12::G1Projective<Config<H>>;
pub type G2Affine<H> = bls12::G2Affine<Config<H>>;
pub type G2Projective<H> = bls12::G2Projective<Config<H>>;

use crate::*;
use ark_ff::Fp12;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate};
use ark_std::{io::Cursor, marker::PhantomData, vec, vec::Vec};
use sp_ark_models::{
    bls12::{Bls12, Bls12Config, G1Prepared, G2Prepared, TwistType},
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
};
use sp_ark_utils::serialize_argument;

pub mod g1;
pub mod g2;
pub(crate) mod util;

use crate::fq;
use ark_bls12_381::{fq::Fq, fq12, fq2, fq6};

#[cfg(test)]
mod tests;

pub use self::{
    g1::{G1Affine, G1Projective},
    g2::{G2Affine, G2Projective},
};

pub struct Config<H: HostFunctions>(PhantomData<fn() -> H>);

pub trait HostFunctions: 'static {
    fn bls12_381_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8>;
    fn bls12_381_final_exponentiation(f12: Vec<u8>) -> Vec<u8>;
    fn bls12_381_msm_g1(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8>;
    fn bls12_381_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bls12_381_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bls12_381_msm_g2(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8>;
    fn bls12_381_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bls12_381_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
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
        let a: Vec<Vec<u8>> = a
            .into_iter()
            .map(|elem| {
                let elem: <Bls12<Self> as Pairing>::G1Prepared = elem.into();
                let elem: <Bls12<Self> as Pairing>::G1Affine = elem.into();
                let mut serialized_result = vec![0u8; elem.serialized_size(Compress::No)];
                let mut cursor = Cursor::new(&mut serialized_result[..]);
                <<Bls12<Self> as Pairing>::G1Affine as CanonicalSerialize>::serialize_uncompressed(&elem, cursor);
                serialized_result
            })
            .collect();
        let b = b
            .into_iter()
            .map(|elem| {
                let elem: <Bls12<Self> as Pairing>::G2Prepared = elem.into();
                let elem: <Bls12<Self> as Pairing>::G2Affine = elem.into();
                let mut serialized_result = vec![0u8; elem.serialized_size(Compress::No)];
                let mut cursor = Cursor::new(&mut serialized_result[..]);
                <<Bls12<Self> as Pairing>::G2Affine as CanonicalSerialize>::serialize_uncompressed(&elem, cursor);
                serialized_result
            })
            .collect();

        let res = H::bls12_381_multi_miller_loop(a, b);

        let cursor = Cursor::new(&res[..]);
        let f: <Bls12<Self> as Pairing>::TargetField =
            Fp12::deserialize_with_mode(cursor, Compress::No, Validate::No).unwrap();
        MillerLoopOutput(f)
    }

    fn final_exponentiation(
        f: MillerLoopOutput<Bls12<Self>>,
    ) -> Option<PairingOutput<Bls12<Self>>> {
        let target = f.0;
        let serialized_target = serialize_argument(target);

        let result = H::bls12_381_final_exponentiation(serialized_target);

        let cursor = Cursor::new(&result[..]);
        let result =
            PairingOutput::<Bls12<Self>>::deserialize_with_mode(cursor, Compress::No, Validate::No)
                .unwrap();

        Some(result)
    }
}

pub type Bls12_381<H> = Bls12<Config<H>>;

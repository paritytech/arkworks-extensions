use ark_std::{marker::PhantomData, vec::Vec};
use codec::{Decode, Encode};
use sp_ark_models::{
    bls12::{Bls12, Bls12Config, G1Prepared, G2Prepared, TwistType},
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
};

pub mod g1;
pub mod g2;
pub(crate) mod util;

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
        let a = a.into_iter().map(|el| {
            let el: <Bls12<Self> as Pairing>::G1Prepared = el.into();
            el
        });
        let a: Vec<u8> =
            ark_scale::iter_ark_to_ark_bytes::<<Bls12<Self> as Pairing>::G1Prepared, _, _>(
                a, HOST_CALL,
            )
            .unwrap();
        let b = b.into_iter().map(|el| {
            let el: <Bls12<Self> as Pairing>::G2Prepared = el.into();
            el
        });
        let b: Vec<u8> =
            ark_scale::iter_ark_to_ark_bytes::<<Bls12<Self> as Pairing>::G2Prepared, _, _>(
                b, HOST_CALL,
            )
            .unwrap();

        let result = H::bls12_381_multi_miller_loop(a.encode(), b.encode()).unwrap();

        let result = <ArkScale<<Bls12<Self> as Pairing>::TargetField> as Decode>::decode(
            &mut result.as_slice(),
        );
        MillerLoopOutput(result.unwrap().0)
    }

    fn final_exponentiation(
        f: MillerLoopOutput<Bls12<Self>>,
    ) -> Option<PairingOutput<Bls12<Self>>> {
        let target: ArkScale<<Bls12<Self> as Pairing>::TargetField> = f.0.into();

        let result = H::bls12_381_final_exponentiation(target.encode()).unwrap();

        let result =
            <ArkScale<PairingOutput<Bls12<Self>>> as Decode>::decode(&mut result.as_slice());

        result.ok().map(|res| res.0)
    }
}

pub type Bls12_381<H> = Bls12<Config<H>>;

use crate::HostFunctions;

use ark_algebra_test_templates::*;
use ark_bls12_377::{
    g1::Config as ArkG1Config, g2::Config as ArkG2Config, Bls12_377 as ArkBls12_377,
};
use ark_ec::pairing::PairingOutput;

struct Mock;

impl HostFunctions for Mock {
    fn bls12_377_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::multi_miller_loop_generic::<ArkBls12_377>(a, b)
    }
    fn bls12_377_final_exponentiation(f: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::final_exponentiation_generic::<ArkBls12_377>(f)
    }
    fn bls12_377_msm_g1(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::msm_sw_generic::<ArkG1Config>(bases, scalars)
    }
    fn bls12_377_msm_g2(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::msm_sw_generic::<ArkG2Config>(bases, scalars)
    }
    fn bls12_377_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::mul_projective_generic::<ArkG1Config>(base, scalar)
    }
    fn bls12_377_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::mul_projective_generic::<ArkG2Config>(base, scalar)
    }
}

test_group!(g1; crate::G1Projective<Mock>; sw);
test_group!(g2; crate::G2Projective<Mock>; sw);
test_group!(pairing_output; PairingOutput<ArkBls12_377>; msm);
test_pairing!(pairing; crate::Bls12_377<super::Mock>);

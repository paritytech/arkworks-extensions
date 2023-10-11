use crate::CurveHooks;

use ark_algebra_test_templates::*;
use ark_bw6_761::{g1::Config as ArkG1Config, g2::Config as ArkG2Config, BW6_761 as ArkBW6_761};

struct Mock;

impl CurveHooks for Mock {
    fn bw6_761_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::multi_miller_loop_generic::<ArkBW6_761>(a, b)
    }
    fn bw6_761_final_exponentiation(f: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::final_exponentiation_generic::<ArkBW6_761>(f)
    }
    fn bw6_761_msm_g1(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::msm_sw_generic::<ArkG1Config>(bases, scalars)
    }
    fn bw6_761_msm_g2(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::msm_sw_generic::<ArkG2Config>(bases, scalars)
    }
    fn bw6_761_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::mul_projective_generic::<ArkG1Config>(base, scalar)
    }
    fn bw6_761_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::mul_projective_generic::<ArkG2Config>(base, scalar)
    }
}

test_group!(g1; crate::G1Projective<Mock>; sw);
test_group!(g2; crate::G2Projective<Mock>; sw);
test_group!(pairing_output; sp_ark_models::pairing::PairingOutput<ArkBW6_761>; msm);
test_pairing!(pairing; crate::BW6_761<super::Mock>);

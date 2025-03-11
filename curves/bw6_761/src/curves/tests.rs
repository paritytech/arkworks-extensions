use crate::CurveHooks;

use ark_algebra_test_templates::*;
use ark_bw6_761::{g1::Config as ArkG1Config, g2::Config as ArkG2Config, BW6_761 as ArkBW6_761};
use ark_models_ext::{
    pairing::{Pairing, PairingOutput},
    CurveConfig,
};

struct TestHooks;

type BW6_761 = crate::BW6_761<TestHooks>;
type G1Projective = crate::G1Projective<TestHooks>;
type G2Projective = crate::G2Projective<TestHooks>;
type G1Affine = crate::G1Affine<TestHooks>;
type G2Affine = crate::G2Affine<TestHooks>;
type G1Config = crate::g1::Config<TestHooks>;
type G2Config = crate::g2::Config<TestHooks>;

impl CurveHooks for TestHooks {
    fn multi_miller_loop(
        g1: impl Iterator<Item = <BW6_761 as Pairing>::G1Prepared>,
        g2: impl Iterator<Item = <BW6_761 as Pairing>::G2Prepared>,
    ) -> <BW6_761 as Pairing>::TargetField {
        test_utils::multi_miller_loop_generic::<BW6_761, ArkBW6_761>(g1, g2)
    }

    fn final_exponentiation(
        target: <BW6_761 as Pairing>::TargetField,
    ) -> <BW6_761 as Pairing>::TargetField {
        test_utils::final_exponentiation_generic::<BW6_761, ArkBW6_761>(target)
    }

    fn msm_g1(
        bases: &[G1Affine],
        scalars: &[<G1Config as CurveConfig>::ScalarField],
    ) -> G1Projective {
        test_utils::msm_sw_generic::<G1Config, ArkG1Config>(bases, scalars)
    }

    fn msm_g2(
        bases: &[G2Affine],
        scalars: &[<G2Config as CurveConfig>::ScalarField],
    ) -> G2Projective {
        test_utils::msm_sw_generic::<G2Config, ArkG2Config>(bases, scalars)
    }

    fn mul_projective_g1(base: &G1Projective, scalar: &[u64]) -> G1Projective {
        test_utils::mul_projective_sw_generic::<G1Config, ArkG1Config>(base, scalar)
    }

    fn mul_projective_g2(base: &G2Projective, scalar: &[u64]) -> G2Projective {
        test_utils::mul_projective_sw_generic::<G2Config, ArkG2Config>(base, scalar)
    }
}

test_group!(g1; G1Projective; sw);
test_group!(g2; G2Projective; sw);
test_group!(pairing_output; PairingOutput<BW6_761>; msm);
test_pairing!(pairing; crate::BW6_761<super::TestHooks>);

use crate::CurveHooks;
use ark_algebra_test_templates::*;
use ark_bls12_377::{
    g1::Config as ArkG1Config, g2::Config as ArkG2Config, Bls12_377 as ArkBls12_377,
};
use ark_models_ext::{
    pairing::{Pairing, PairingOutput},
    CurveConfig,
};

struct TestHooks;

type Bls12_377 = crate::Bls12_377<TestHooks>;
type G1Projective = crate::G1Projective<TestHooks>;
type G2Projective = crate::G2Projective<TestHooks>;
type G1Affine = crate::G1Affine<TestHooks>;
type G2Affine = crate::G2Affine<TestHooks>;
type G1Config = crate::g1::Config<TestHooks>;
type G2Config = crate::g2::Config<TestHooks>;

impl CurveHooks for TestHooks {
    fn bls12_377_multi_miller_loop(
        g1: impl Iterator<Item = <Bls12_377 as Pairing>::G1Prepared>,
        g2: impl Iterator<Item = <Bls12_377 as Pairing>::G2Prepared>,
    ) -> Result<<Bls12_377 as Pairing>::TargetField, ()> {
        test_utils::multi_miller_loop_generic2::<Bls12_377, ArkBls12_377>(g1, g2)
    }

    fn bls12_377_final_exponentiation(
        target: <Bls12_377 as Pairing>::TargetField,
    ) -> Result<<Bls12_377 as Pairing>::TargetField, ()> {
        test_utils::final_exponentiation_generic2::<Bls12_377, ArkBls12_377>(target)
    }

    fn bls12_377_msm_g1(
        bases: &[G1Affine],
        scalars: &[<G1Config as CurveConfig>::ScalarField],
    ) -> Result<G1Projective, ()> {
        test_utils::msm_sw_generic2::<G1Config, ArkG1Config>(bases, scalars)
    }

    fn bls12_377_msm_g2(
        bases: &[G2Affine],
        scalars: &[<G2Config as CurveConfig>::ScalarField],
    ) -> Result<G2Projective, ()> {
        test_utils::msm_sw_generic2::<G2Config, ArkG2Config>(bases, scalars)
    }

    fn bls12_377_mul_projective_g1(
        base: &G1Projective,
        scalar: &[u64],
    ) -> Result<G1Projective, ()> {
        test_utils::mul_projective_generic2::<G1Config, ArkG1Config>(base, scalar)
    }

    fn bls12_377_mul_projective_g2(
        base: &G2Projective,
        scalar: &[u64],
    ) -> Result<G2Projective, ()> {
        test_utils::mul_projective_generic2::<G2Config, ArkG2Config>(base, scalar)
    }
}

test_group!(g1; G1Projective; sw);
test_group!(g2; G2Projective; sw);
test_group!(pairing_output; PairingOutput<Bls12_377>; msm);
test_pairing!(pairing; crate::Bls12_377<super::TestHooks>);

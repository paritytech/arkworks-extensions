use crate::{CurveHooks, EdwardsConfig};

use ark_algebra_test_templates::*;
use ark_ed_on_bls12_377::EdwardsConfig as ArkEdwardsConfig;
use ark_models_ext::{Affine, CurveConfig, Projective};
use ark_std::vec::Vec;

struct TestHooks;

impl CurveHooks for TestHooks {
    fn ed_on_bls12_377_msm(
        bases: &[Affine<EdwardsConfig<Self>>],
        scalars: &[<EdwardsConfig<Self> as CurveConfig>::ScalarField],
    ) -> Result<Projective<EdwardsConfig<Self>>, ()> {
        test_utils::msm_te_generic2::<EdwardsConfig<TestHooks>, ArkEdwardsConfig>(bases, scalars)
    }

    fn ed_on_bls12_377_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::mul_projective_te_generic::<ArkEdwardsConfig>(base, scalar)
    }
}

test_group!(te; crate::EdwardsProjective<TestHooks>; te);

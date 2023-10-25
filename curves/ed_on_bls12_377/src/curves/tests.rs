use crate::{CurveHooks, EdwardsAffine, EdwardsConfig, EdwardsProjective};

use ark_algebra_test_templates::*;
use ark_ed_on_bls12_377::EdwardsConfig as ArkEdwardsConfig;
use ark_models_ext::CurveConfig;

struct TestHooks;

impl CurveHooks for TestHooks {
    fn ed_on_bls12_377_msm(
        bases: &[EdwardsAffine<Self>],
        scalars: &[<EdwardsConfig<Self> as CurveConfig>::ScalarField],
    ) -> Result<EdwardsProjective<Self>, ()> {
        test_utils::msm_te_generic2::<EdwardsConfig<Self>, ArkEdwardsConfig>(bases, scalars)
    }

    fn ed_on_bls12_377_mul_projective(
        base: &EdwardsProjective<Self>,
        scalar: &[u64],
    ) -> Result<EdwardsProjective<Self>, ()> {
        test_utils::mul_projective_te_generic2::<EdwardsConfig<Self>, ArkEdwardsConfig>(
            base, scalar,
        )
    }
}

test_group!(te; crate::EdwardsProjective<TestHooks>; te);

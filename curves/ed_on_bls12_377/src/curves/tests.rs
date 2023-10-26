use crate::CurveHooks;

use ark_algebra_test_templates::*;
use ark_ed_on_bls12_377::EdwardsConfig as ArkConfig;
use ark_models_ext::CurveConfig;

struct TestHooks;

type Config = crate::EdwardsConfig<TestHooks>;
type Affine = crate::EdwardsAffine<TestHooks>;
type Projective = crate::EdwardsProjective<TestHooks>;

impl CurveHooks for TestHooks {
    fn ed_on_bls12_377_msm(
        bases: &[Affine],
        scalars: &[<Config as CurveConfig>::ScalarField],
    ) -> Result<Projective, ()> {
        test_utils::msm_te_generic::<Config, ArkConfig>(bases, scalars)
    }

    fn ed_on_bls12_377_mul_projective(base: &Projective, scalar: &[u64]) -> Result<Projective, ()> {
        test_utils::mul_projective_te_generic::<Config, ArkConfig>(base, scalar)
    }
}

test_group!(te; Projective; te);

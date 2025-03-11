use crate::CurveHooks;

use ark_algebra_test_templates::*;
use ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig as ArkConfig;
use ark_models_ext::CurveConfig;

pub struct TestHooks;

type Config = crate::BandersnatchConfig<TestHooks>;
type EdwardsAffine = crate::EdwardsAffine<TestHooks>;
type EdwardsProjective = crate::EdwardsProjective<TestHooks>;
type SWAffine = crate::SWAffine<TestHooks>;
type SWProjective = crate::SWProjective<TestHooks>;

impl CurveHooks for TestHooks {
    fn msm_te(
        bases: &[EdwardsAffine],
        scalars: &[<Config as CurveConfig>::ScalarField],
    ) -> EdwardsProjective {
        test_utils::msm_te_generic::<Config, ArkConfig>(bases, scalars)
    }

    fn mul_projective_te(base: &EdwardsProjective, scalar: &[u64]) -> EdwardsProjective {
        test_utils::mul_projective_te_generic::<Config, ArkConfig>(base, scalar)
    }

    fn msm_sw(
        bases: &[SWAffine],
        scalars: &[<Config as CurveConfig>::ScalarField],
    ) -> SWProjective {
        test_utils::msm_sw_generic::<Config, ArkConfig>(bases, scalars)
    }

    fn mul_projective_sw(base: &SWProjective, scalar: &[u64]) -> SWProjective {
        test_utils::mul_projective_sw_generic::<Config, ArkConfig>(base, scalar)
    }
}

test_group!(te; EdwardsProjective; te);
test_group!(sw; SWProjective; sw);

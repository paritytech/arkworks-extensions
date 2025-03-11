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
    fn ed_on_bls12_381_bandersnatch_te_msm(
        bases: &[EdwardsAffine],
        scalars: &[<Config as CurveConfig>::ScalarField],
    ) -> EdwardsProjective {
        test_utils::msm_te_generic::<Config, ArkConfig>(bases, scalars)
    }

    fn ed_on_bls12_381_bandersnatch_te_mul_projective(
        base: &EdwardsProjective,
        scalar: &[u64],
    ) -> EdwardsProjective {
        test_utils::mul_projective_te_generic::<Config, ArkConfig>(base, scalar)
    }

    fn ed_on_bls12_381_bandersnatch_sw_msm(
        bases: &[SWAffine],
        scalars: &[<Config as CurveConfig>::ScalarField],
    ) -> SWProjective {
        test_utils::msm_sw_generic::<Config, ArkConfig>(bases, scalars)
    }

    fn ed_on_bls12_381_bandersnatch_sw_mul_projective(
        base: &SWProjective,
        scalar: &[u64],
    ) -> SWProjective {
        test_utils::mul_projective_sw_generic::<Config, ArkConfig>(base, scalar)
    }
}

test_group!(te; EdwardsProjective; te);
test_group!(sw; SWProjective; sw);

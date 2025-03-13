use crate::CurveHooks;

use ark_algebra_test_templates::*;
use ark_models_ext::CurveConfig;
use ark_secp256k1::Config as ArkConfig;

pub struct TestHooks;

type Config = crate::Secp256k1Config<TestHooks>;
type Affine = crate::Affine<TestHooks>;
type Projective = crate::Projective<TestHooks>;

impl CurveHooks for TestHooks {
    fn msm(bases: &[Affine], scalars: &[<Config as CurveConfig>::ScalarField]) -> Projective {
        test_utils::msm_sw_generic::<Config, ArkConfig>(bases, scalars)
    }

    fn mul_projective(base: &Projective, scalar: &[u64]) -> Projective {
        test_utils::mul_projective_sw_generic::<Config, ArkConfig>(base, scalar)
    }
}

test_group!(sw; Projective; sw);

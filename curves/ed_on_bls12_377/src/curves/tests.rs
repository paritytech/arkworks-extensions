use crate::HostFunctions;

use ark_algebra_test_templates::*;
use ark_ed_on_bls12_377::EdwardsConfig as ArkEdwardsConfig;

struct Mock;

impl HostFunctions for Mock {
    fn ed_on_bls12_377_msm(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::msm_te_generic::<ArkEdwardsConfig>(bases, scalars)
    }
    fn ed_on_bls12_377_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        test_utils::mul_projective_te_generic::<ArkEdwardsConfig>(base, scalar)
    }
}

test_group!(te; crate::EdwardsProjective<Mock>; te);

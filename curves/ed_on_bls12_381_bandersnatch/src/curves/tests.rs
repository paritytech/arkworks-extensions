use crate::CurveHooks;

use ark_algebra_test_templates::*;
use ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig as ArkBandersnatchConfig;

pub struct Mock;

impl CurveHooks for Mock {
    fn ed_on_bls12_381_bandersnatch_te_msm(
        bases: Vec<u8>,
        scalars: Vec<u8>,
    ) -> Result<Vec<u8>, ()> {
        test_utils::msm_te_generic::<ArkBandersnatchConfig>(bases, scalars)
    }
    fn ed_on_bls12_381_bandersnatch_sw_msm(
        bases: Vec<u8>,
        scalars: Vec<u8>,
    ) -> Result<Vec<u8>, ()> {
        test_utils::msm_sw_generic::<ArkBandersnatchConfig>(bases, scalars)
    }
    fn ed_on_bls12_381_bandersnatch_te_mul_projective(
        base: Vec<u8>,
        scalar: Vec<u8>,
    ) -> Result<Vec<u8>, ()> {
        test_utils::mul_projective_te_generic::<ArkBandersnatchConfig>(base, scalar)
    }
    fn ed_on_bls12_381_bandersnatch_sw_mul_projective(
        base: Vec<u8>,
        scalar: Vec<u8>,
    ) -> Result<Vec<u8>, ()> {
        test_utils::mul_projective_generic::<ArkBandersnatchConfig>(base, scalar)
    }
}

test_group!(te; crate::EdwardsProjective<Mock>; te);

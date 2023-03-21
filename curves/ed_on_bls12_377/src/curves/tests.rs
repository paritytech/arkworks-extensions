#![cfg_attr(not(feature = "std"), no_std)]
use crate::HostFunctions;
use ark_algebra_test_templates::*;
use ark_std::vec::Vec;

pub struct Host {}

impl HostFunctions for Host {
    fn ed_on_bls12_377_msm(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_377_msm(bases, scalars)
    }
    fn ed_on_bls12_377_mul_affine(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_377_mul_affine(base, scalar)
    }
    fn ed_on_bls12_377_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_377_mul_projective(base, scalar)
    }
}

test_group!(te; crate::EdwardsProjective<super::Host>; te);

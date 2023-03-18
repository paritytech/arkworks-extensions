#![cfg_attr(not(feature = "std"), no_std)]
use crate::HostFunctions;
use ark_algebra_test_templates::*;
use ark_std::vec::Vec;
use sp_arkworks::PairingError;

#[derive(PartialEq, Eq)]
pub struct Host;

impl HostFunctions for Host {
    fn bw6_761_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, PairingError> {
        sp_io::elliptic_curves::bw6_761_multi_miller_loop(a, b)
    }
    fn bw6_761_final_exponentiation(f12: Vec<u8>) -> Result<Vec<u8>, PairingError> {
        sp_io::elliptic_curves::bw6_761_final_exponentiation(f12)
    }
    fn bw6_761_msm_g1(bases: Vec<u8>, bigints: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_msm_g1(bases, bigints)
    }
    fn bw6_761_msm_g2(bases: Vec<u8>, bigints: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_msm_g2(bases, bigints)
    }
}

test_group!(g1; crate::g1::G1Projective<super::Host>; sw);
test_group!(g2; crate::g2::G2Projective<super::Host>; sw);
test_group!(pairing_output; sp_ark_models::pairing::PairingOutput<crate::BW6_761<super::Host>>; msm);
test_pairing!(pairing; crate::BW6_761<super::Host>);

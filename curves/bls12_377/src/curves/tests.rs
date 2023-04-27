#![cfg_attr(not(feature = "std"), no_std)]
use ark_algebra_test_templates::*;

test_group!(g1; crate::curves::g1::G1Projective; sw);
test_group!(g2; crate::curves::g2::G2Projective; sw);
test_group!(pairing_output; ark_ec::pairing::PairingOutput<crate::curves::Bls12_377>; msm);
test_pairing!(pairing; crate::curves::Bls12_377);

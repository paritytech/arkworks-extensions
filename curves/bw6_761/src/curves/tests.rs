#![cfg_attr(not(feature = "std"), no_std)]
use ark_algebra_test_templates::*;

test_group!(g1; crate::g1::G1Projective; sw);
test_group!(g2; crate::g2::G2Projective; sw);
test_group!(pairing_output; sp_ark_models::pairing::PairingOutput<crate::BW6_761>; msm);
test_pairing!(pairing; crate::BW6_761);

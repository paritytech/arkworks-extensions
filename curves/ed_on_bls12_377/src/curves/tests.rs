#![cfg_attr(not(feature = "std"), no_std)]
use ark_algebra_test_templates::*;

test_group!(te; crate::EdwardsProjective; te);

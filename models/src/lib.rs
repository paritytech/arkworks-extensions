#![cfg_attr(not(feature = "std"), no_std)]
// Temporary fix to make clippy happy with implementation of clone on copy types
// provided by "derivative" crate.
#![allow(clippy::non_canonical_clone_impl)]

pub use ark_ec::{
    scalar_mul, scalar_mul::*, twisted_edwards, twisted_edwards::*, AffineRepr, CurveGroup,
    PrimeGroup, VariableBaseMSM,
};
pub mod models;
pub use models::*;

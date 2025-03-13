//! This library implements the secp256k1 curve.
//! Source: <https://en.bitcoin.it/wiki/Secp256k1>
//!
//! Curve information:
//! * Base field: q =
//!   115792089237316195423570985008687907853269984665640564039457584007908834671663
//! * Scalar field: r =
//!   115792089237316195423570985008687907852837564279074904382605163141518161494337
//! * Curve equation: y^2 = x^3 + 7

#![cfg_attr(not(feature = "std"), no_std)]

pub mod curves;

pub use ark_secp256k1::{fq, fq::*, fr, fr::*};
pub use curves::*;

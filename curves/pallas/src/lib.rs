//! This library implements the prime-order curve Pallas, generated by
//! [Daira Hopwood](https://github.com/zcash/pasta). The main feature of this
//! curve is that it forms a cycle with Vesta, i.e. its scalar field and base
//! field respectively are the base field and scalar field of Vesta.
//!
//!
//! Curve information:
//! * Base field: q =
//!   28948022309329048855892746252171976963363056481941560715954676764349967630337
//! * Scalar field: r =
//!   28948022309329048855892746252171976963363056481941647379679742748393362948097
//! * Curve equation: y^2 = x^3 + 5
//! * Valuation(q - 1, 2) = 32
//! * Valuation(r - 1, 2) = 32

#![cfg_attr(not(feature = "std"), no_std)]

pub mod curves;

pub use ark_pallas::{fq, fq::*, fr, fr::*};
pub use curves::*;

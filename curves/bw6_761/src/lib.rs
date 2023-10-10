//! This library implements the BW6_761 curve generated in [\[EG20\]](https://eprint.iacr.org/2020/351).
//! The name denotes that it is a curve generated using the Brezing--Weng
//! method, and that its embedding degree is 6.
//! The main feature of this curve is that the scalar field equals the base
//! field of the BLS12_377 curve.
//!
//! Curve information:
//! * Base field: q = 6891450384315732539396789682275657542479668912536150109513790160209623422243491736087683183289411687640864567753786613451161759120554247759349511699125301598951605099378508850372543631423596795951899700429969112842764913119068299
//! * Scalar field: r = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
//! * valuation(q - 1, 2) = 1
//! * valuation(r - 1, 2) = 46
//!
//! G1 curve equation: y^2 = x^3 + ax + b, where
//! * a = 0,
//! * b = -1,
//!
//! G2 curve equation: y^2 = x^3 + Ax + B
//! * A = 0
//! * B = 4

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

pub mod curves;

pub use ark_bw6_761::{fq, fq::*, fq3, fq3::*, fq6, fq6::*, fr, fr::*};
pub use curves::*;

use ark_scale::{
    ark_serialize::{Compress, Validate},
    Usage,
};

#[cfg(feature = "scale-no-compress")]
const SCALE_COMPRESS: Compress = Compress::No;
#[cfg(not(feature = "scale-no-compress"))]
const SCALE_COMPRESS: Compress = Compress::Yes;

#[cfg(feature = "scale-no-validate")]
const SCALE_VALIDATE: Validate = Validate::No;
#[cfg(not(feature = "scale-no-validate"))]
const SCALE_VALIDATE: Validate = Validate::Yes;

/// SCALE codec usage settings.
///
/// Determines whether compression and validation has been enabled for SCALE codec
/// with respect to ARK related types.
pub const SCALE_USAGE: Usage = ark_scale::make_usage(SCALE_COMPRESS, SCALE_VALIDATE);

type ArkScale<T> = ark_scale::ArkScale<T, SCALE_USAGE>;

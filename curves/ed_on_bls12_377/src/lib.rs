//! This library implements a twisted Edwards curve whose base field is the
//! scalar field of the curve BLS12-377.  This allows defining cryptographic
//! primitives that use elliptic curves over the scalar field of the latter
//! curve. This curve was generated as part of the paper [\[BCGMMW20, “Zexe”\]](https://eprint.iacr.org/2018/962).
//!
//! Curve information:
//! * Base field: q = 8444461749428370424248824938781546531375899335154063827935233455917409239041
//! * Scalar field: r = 2111115437357092606062206234695386632838870926408408195193685246394721360383
//! * Valuation(q - 1, 2) = 47
//! * Valuation(r - 1, 2) = 1
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = -1
//!    * d = 3021

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

pub use ark_ed_on_bls12_377::{fq, fq::*, fr, fr::*};
pub use curves::*;

#[cfg(feature = "r1cs")]
pub use ark_ed_on_bls12_377::constraints;

use ark_scale::ark_serialize::{Compress, Validate};

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
pub const SCALE_USAGE: u8 = ark_scale::make_usage(SCALE_COMPRESS, SCALE_VALIDATE);

type ArkScale<T> = ark_scale::ArkScale<T, SCALE_USAGE>;

//! This library implements the Bendersnatch curve, a twisted Edwards curve
//! whose base field is the scalar field of the curve BLS12-381. This allows
//! defining cryptographic primitives that use elliptic curves over the scalar
//! field of the latter curve. This curve was generated by Simon Masson from
//! Anoma, and Antonio Sanso from Ethereum Foundation, and is also known as [bandersnatch](https://ethresear.ch/t/introducing-bandersnatch-a-fast-elliptic-curve-built-over-the-bls12-381-scalar-field/9957).
//!
//! See [here](https://github.com/asanso/Bandersnatch/blob/main/README.md) for the specification of the curve.
//! There was also a Python implementation [here](https://github.com/asanso/Bandersnatch/).
//!
//! Curve information:
//! * Base field: q =
//!   52435875175126190479447740508185965837690552500527637822603658699938581184513
//! * Scalar field: r =
//!   13108968793781547619861935127046491459309155893440570251786403306729687672801
//! * Valuation(q - 1, 2) = 32
//! * Valuation(r - 1, 2) = 5
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = -5
//!    * d = 45022363124591815672509500913686876175488063829319466900776701791074614335719

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code
)]
#![allow(clippy::result_unit_err)]

pub mod curves;

pub use ark_ed_on_bls12_381_bandersnatch::{fq, fq::*, fr, fr::*};
pub use curves::*;

#[cfg(feature = "r1cs")]
pub use ark_ed_on_bls12_381_bandersnatch::constraints;

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

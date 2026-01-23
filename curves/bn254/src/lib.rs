//! This library implements the BN254 curve.
//! The name denotes that it is a Barreto-Naehrig curve of embedding degree
//! 12, defined over a 254-bit (prime) field. This curve is also known as
//! alt-bn128 or bn256.
//!
//! Curve information:
//! * Base field: q = 21888242871839275222246405745257275088696311157297823662689037894645226208583
//! * Scalar field: r = 21888242871839275222246405745257275088548364400416034343698204186575808495617
//! * valuation(q - 1, 2) = 1
//! * valuation(r - 1, 2) = 28
//! * G1 curve equation: y^2 = x^3 + 3
//! * G2 curve equation: y^2 = x^3 + 3/(u+9) where Fq2 = Fq[u]/(u^2+1)

#![cfg_attr(not(feature = "std"), no_std)]

mod curves;

pub use ark_bn254::{fq, fq::*, fq12, fq12::*, fq2, fq2::*, fq6, fq6::*, fr, fr::*};
pub use curves::*;

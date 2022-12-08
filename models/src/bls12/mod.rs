use ark_ec::{
	models::CurveConfig,
	pairing::{MillerLoopOutput, Pairing, PairingOutput},
	AffineRepr,
};
use ark_ff::{
	fields::{
		fp12_2over3over2::{Fp12, Fp12Config},
		fp2::Fp2Config,
		fp6_3over2::Fp6Config,
		Fp2,
	},
	CyclotomicMultSubgroup, PrimeField,
};
use ark_std::{marker::PhantomData, vec::Vec};
use derivative::Derivative;

use crate::short_weierstrass::SWCurveConfig;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// A particular BLS12 group can have G2 being either a multiplicative or a
/// divisive twist.
pub enum TwistType {
	M,
	D,
}

pub trait Bls12Parameters: 'static + Sized {
	/// Parameterizes the BLS12 family.
	const X: &'static [u64];
	/// Is `Self::X` negative?
	const X_IS_NEGATIVE: bool;
	/// What kind of twist is this?
	const TWIST_TYPE: TwistType;

	type Fp: PrimeField + Into<<Self::Fp as PrimeField>::BigInt>;
	type Fp2Config: Fp2Config<Fp = Self::Fp>;
	type Fp6Config: Fp6Config<Fp2Config = Self::Fp2Config>;
	type Fp12Config: Fp12Config<Fp6Config = Self::Fp6Config>;
	type G1Parameters: SWCurveConfig<BaseField = Self::Fp>;
	type G2Parameters: SWCurveConfig<
		BaseField = Fp2<Self::Fp2Config>,
		ScalarField = <Self::G1Parameters as CurveConfig>::ScalarField,
	>;

	fn multi_miller_loop(
		a_vec: impl IntoIterator<Item = impl Into<G1Prepared<Self>>>,
		b_vec: impl IntoIterator<Item = impl Into<G2Prepared<Self>>>,
	) -> MillerLoopOutput<Bls12<Self>>;
	fn final_exponentiation(f: MillerLoopOutput<Bls12<Self>>)
		-> Option<PairingOutput<Bls12<Self>>>;
}

pub mod g1;
pub mod g2;

pub use self::{
	g1::{G1Affine, G1Prepared, G1Projective},
	g2::{G2Affine, G2Prepared, G2Projective},
};

#[derive(Derivative)]
#[derivative(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Bls12<P: Bls12Parameters>(PhantomData<fn() -> P>);

impl<P: Bls12Parameters> Bls12<P> {
	// Evaluate the line function at point p.
	fn ell(f: &mut Fp12<P::Fp12Config>, coeffs: &g2::EllCoeff<P>, p: &G1Affine<P>) {
		let mut c0 = coeffs.0;
		let mut c1 = coeffs.1;
		let mut c2 = coeffs.2;
		let (px, py) = p.xy().unwrap();

		match P::TWIST_TYPE {
			TwistType::M => {
				c2.mul_assign_by_fp(py);
				c1.mul_assign_by_fp(px);
				f.mul_by_014(&c0, &c1, &c2);
			},
			TwistType::D => {
				c0.mul_assign_by_fp(py);
				c1.mul_assign_by_fp(px);
				f.mul_by_034(&c0, &c1, &c2);
			},
		}
	}

	// Exponentiates `f` by `Self::X`, and stores the result in `result`.
	fn exp_by_x(f: &Fp12<P::Fp12Config>, result: &mut Fp12<P::Fp12Config>) {
		*result = f.cyclotomic_exp(P::X);
		if P::X_IS_NEGATIVE {
			result.cyclotomic_inverse_in_place();
		}
	}
}

impl<P: Bls12Parameters> Pairing for Bls12<P> {
	type BaseField = <P::G1Parameters as CurveConfig>::BaseField;
	type ScalarField = <P::G1Parameters as CurveConfig>::ScalarField;
	type G1 = G1Projective<P>;
	type G1Affine = G1Affine<P>;
	type G1Prepared = G1Prepared<P>;
	type G2 = G2Projective<P>;
	type G2Affine = G2Affine<P>;
	type G2Prepared = G2Prepared<P>;
	type TargetField = Fp12<P::Fp12Config>;

	fn multi_miller_loop(
		a: impl IntoIterator<Item = impl Into<Self::G1Prepared>>,
		b: impl IntoIterator<Item = impl Into<Self::G2Prepared>>,
	) -> MillerLoopOutput<Self> {
		P::multi_miller_loop(a, b)
	}

	fn final_exponentiation(f: MillerLoopOutput<Self>) -> Option<PairingOutput<Self>> {
		P::final_exponentiation(f)
	}
}

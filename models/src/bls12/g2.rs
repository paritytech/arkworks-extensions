use crate::{
	bls12::Bls12Parameters,
	short_weierstrass::{Affine, Projective},
};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::Fp2;
use ark_serialize::*;
use ark_std::vec::Vec;
use derivative::Derivative;

pub type G2Affine<P> = Affine<<P as Bls12Parameters>::G2Parameters>;
pub type G2Projective<P> = Projective<<P as Bls12Parameters>::G2Parameters>;

pub(crate) type EllCoeff<P> = (
	Fp2<<P as Bls12Parameters>::Fp2Config>,
	Fp2<<P as Bls12Parameters>::Fp2Config>,
	Fp2<<P as Bls12Parameters>::Fp2Config>,
);

#[derive(Derivative, CanonicalSerialize, CanonicalDeserialize)]
#[derivative(
	Clone(bound = "P: Bls12Parameters"),
	Debug(bound = "P: Bls12Parameters"),
	PartialEq(bound = "P: Bls12Parameters"),
	Eq(bound = "P: Bls12Parameters")
)]
pub struct G2Prepared<P: Bls12Parameters>(pub G2Affine<P>);

impl<P: Bls12Parameters> From<G2Affine<P>> for G2Prepared<P> {
	fn from(other: G2Affine<P>) -> Self {
		G2Prepared(other)
	}
}

impl<P: Bls12Parameters> From<G2Projective<P>> for G2Prepared<P> {
	fn from(q: G2Projective<P>) -> Self {
		q.into_affine().into()
	}
}

impl<'a, P: Bls12Parameters> From<&'a G2Affine<P>> for G2Prepared<P> {
	fn from(other: &'a G2Affine<P>) -> Self {
		G2Prepared(*other)
	}
}

impl<'a, P: Bls12Parameters> From<&'a G2Projective<P>> for G2Prepared<P> {
	fn from(q: &'a G2Projective<P>) -> Self {
		q.into_affine().into()
	}
}

impl<P: Bls12Parameters> G2Prepared<P> {
	pub fn is_zero(&self) -> bool {
		self.0.is_zero()
	}
}

impl<P: Bls12Parameters> Default for G2Prepared<P> {
	fn default() -> Self {
		G2Prepared(G2Affine::<P>::generator())
	}
}

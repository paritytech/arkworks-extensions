use crate::models::{
    bn::BnConfig,
    short_weierstrass::{Affine, Projective},
};
use ark_ec::{AffineRepr, CurveGroup};
use ark_serialize::*;
use ark_std::vec::Vec;
use derivative::Derivative;

pub type G2Affine<P> = Affine<<P as BnConfig>::G2Config>;
pub type G2Projective<P> = Projective<<P as BnConfig>::G2Config>;

#[derive(Derivative, CanonicalSerialize, CanonicalDeserialize)]
#[derivative(
    Copy(bound = "P: BnConfig"),
    Clone(bound = "P: BnConfig"),
    PartialEq(bound = "P: BnConfig"),
    Eq(bound = "P: BnConfig"),
    Debug(bound = "P: BnConfig")
)]
pub struct G2Prepared<P: BnConfig>(pub G2Affine<P>);

impl<P: BnConfig> From<G2Affine<P>> for G2Prepared<P> {
    fn from(other: G2Affine<P>) -> Self {
        G2Prepared(other)
    }
}

impl<P: BnConfig> From<G2Projective<P>> for G2Prepared<P> {
    fn from(q: G2Projective<P>) -> Self {
        q.into_affine().into()
    }
}

impl<'a, P: BnConfig> From<&'a G2Affine<P>> for G2Prepared<P> {
    fn from(other: &'a G2Affine<P>) -> Self {
        G2Prepared(*other)
    }
}

impl<'a, P: BnConfig> From<&'a G2Projective<P>> for G2Prepared<P> {
    fn from(q: &'a G2Projective<P>) -> Self {
        q.into_affine().into()
    }
}

impl<P: BnConfig> G2Prepared<P> {
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<P: BnConfig> Default for G2Prepared<P> {
    fn default() -> Self {
        G2Prepared(G2Affine::<P>::generator())
    }
}

use ark_ec::{
    twisted_edwards::{Affine, MontCurveConfig, Projective, TECurveConfig},
    CurveConfig,
};
use ark_ed_on_bls12_377::EdwardsConfig as OrgEdConfig;
use ark_ff::MontFp;
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use ark_std::{io::Cursor, marker::PhantomData, vec::Vec};

use sp_ark_utils::serialize_argument;

use crate::fq::Fq;

#[cfg(test)]
mod tests;

pub type EdwardsAffine<H> = Affine<EdwardsConfig<H>>;
pub type EdwardsProjective<H> = Projective<EdwardsConfig<H>>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct EdwardsConfig<H: HostFunctions>(PhantomData<fn() -> H>);

pub trait HostFunctions: 'static {
    fn ed_on_bls12_377_msm(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8>;
    fn ed_on_bls12_377_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn ed_on_bls12_377_mul_affine(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
}

impl<H: HostFunctions> CurveConfig for EdwardsConfig<H> {
    type BaseField = <OrgEdConfig as CurveConfig>::BaseField;
    type ScalarField = <OrgEdConfig as CurveConfig>::ScalarField;

    const COFACTOR: &'static [u64] = <OrgEdConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <OrgEdConfig as CurveConfig>::COFACTOR_INV;
}

/// TODO: peek from upstream once (and if) this is merged:
/// https://github.com/arkworks-rs/curves/pull/150
/// GENERATOR_X =
/// 4497879464030519973909970603271755437257548612157028181994697785683032656389,
const GENERATOR_X: Fq =
    MontFp!("4497879464030519973909970603271755437257548612157028181994697785683032656389");

/// TODO: peek from upstream once (and if) this is merged:
/// https://github.com/arkworks-rs/curves/pull/150
/// GENERATOR_Y =
/// 4357141146396347889246900916607623952598927460421559113092863576544024487809
const GENERATOR_Y: Fq =
    MontFp!("4357141146396347889246900916607623952598927460421559113092863576544024487809");

impl<H: HostFunctions> TECurveConfig for EdwardsConfig<H> {
    const COEFF_A: Self::BaseField = <OrgEdConfig as TECurveConfig>::COEFF_A;
    const COEFF_D: Self::BaseField = <OrgEdConfig as TECurveConfig>::COEFF_D;

    /// Generated randomly
    const GENERATOR: EdwardsAffine<H> = EdwardsAffine::<H>::new_unchecked(GENERATOR_X, GENERATOR_Y);

    type MontCurveConfig = Self;

    /// Multiplication by `a` is just negation.
    /// Is `a` 1 or -1?
    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        -elem
    }

    fn msm(
        bases: &[Affine<Self>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: Vec<Vec<u8>> = bases.iter().map(|elem| serialize_argument(*elem)).collect();
        let scalars: Vec<Vec<u8>> = scalars
            .iter()
            .map(|elem| serialize_argument(*elem))
            .collect();

        let result = H::ed_on_bls12_377_msm(bases, scalars);

        let cursor = Cursor::new(&result[..]);
        let result = Self::deserialize_with_mode(cursor, Compress::Yes, Validate::No).unwrap();
        Ok(result.into())
    }

    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let serialized_base = serialize_argument(*base);
        let serialized_scalar = serialize_argument(scalar);

        let result = H::ed_on_bls12_377_mul_projective(serialized_base, serialized_scalar);

        let cursor = Cursor::new(&result[..]);

        Projective::<Self>::deserialize_with_mode(cursor, Compress::Yes, Validate::No).unwrap()
    }

    fn mul_affine(base: &Affine<Self>, scalar: &[u64]) -> Projective<Self> {
        let serialized_base = serialize_argument(*base);
        let serialized_scalar = serialize_argument(scalar);

        let result = H::ed_on_bls12_377_mul_affine(serialized_base, serialized_scalar);

        let cursor = Cursor::new(&result[..]);

        Projective::<Self>::deserialize_with_mode(cursor, Compress::Yes, Validate::No).unwrap()
    }
}

impl<H: HostFunctions> MontCurveConfig for EdwardsConfig<H> {
    const COEFF_A: Self::BaseField = <OrgEdConfig as MontCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <OrgEdConfig as MontCurveConfig>::COEFF_B;

    type TECurveConfig = Self;
}

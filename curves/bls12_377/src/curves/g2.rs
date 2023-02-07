use ark_bls12_377::g2::{Config as OrgConfig, G2_GENERATOR_X, G2_GENERATOR_Y};
use ark_ec::{
    models::{bls12, CurveConfig},
    short_weierstrass::{Affine as SWAffine, Projective as SWProjective, SWCurveConfig},
};
use ark_serialize::{Compress, Validate};
use ark_std::{io::Cursor, marker::PhantomData, vec::Vec};
use sp_ark_utils::serialize_argument;

use crate::HostFunctions;

pub type G2Affine<H> = bls12::G2Affine<crate::Config<H>>;
pub type G2Projective<H> = bls12::G2Projective<crate::Config<H>>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config<H: HostFunctions>(PhantomData<fn() -> H>);

impl<H: HostFunctions> CurveConfig for Config<H> {
    type BaseField = <OrgConfig as CurveConfig>::BaseField;
    type ScalarField = <OrgConfig as CurveConfig>::ScalarField;

    const COFACTOR: &'static [u64] = <OrgConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <OrgConfig as CurveConfig>::COFACTOR_INV;
}

impl<H: HostFunctions> SWCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <OrgConfig as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <OrgConfig as SWCurveConfig>::COEFF_B;

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const GENERATOR: G2Affine<H> = G2Affine::<H>::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    fn msm(
        bases: &[SWAffine<Self>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<SWProjective<Self>, usize> {
        let bases: Vec<Vec<u8>> = bases.iter().map(|elem| serialize_argument(*elem)).collect();
        let scalars: Vec<Vec<u8>> = scalars
            .iter()
            .map(|elem| serialize_argument(*elem))
            .collect();

        let result = H::bls12_377_msm_g2(bases, scalars);

        let cursor = Cursor::new(&result[..]);
        let result = <Config<H> as SWCurveConfig>::deserialize_with_mode(
            cursor,
            Compress::Yes,
            Validate::No,
        )
        .unwrap();
        Ok(result.into())
    }

    fn mul_projective(base: &SWProjective<Self>, scalar: &[u64]) -> SWProjective<Self> {
        let serialized_base = serialize_argument(*base);
        let serialized_scalar = serialize_argument(scalar);

        let result = H::bls12_377_mul_projective_g2(serialized_base, serialized_scalar);

        let cursor = Cursor::new(&result[..]);
        let result = Self::deserialize_with_mode(cursor, Compress::Yes, Validate::No).unwrap();
        result.into()
    }

    fn mul_affine(base: &SWAffine<Self>, scalar: &[u64]) -> SWProjective<Self> {
        let serialized_base = serialize_argument(*base);
        let serialized_scalar = serialize_argument(scalar);

        let result = H::bls12_377_mul_affine_g2(serialized_base, serialized_scalar);

        let cursor = Cursor::new(&result[..]);
        let result = Self::deserialize_with_mode(cursor, Compress::Yes, Validate::No).unwrap();
        result.into()
    }
}

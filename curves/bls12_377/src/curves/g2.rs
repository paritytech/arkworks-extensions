use crate::{ArkScale, CurveHooks};
use ark_bls12_377::g2::Config as ArkG2Config;
use ark_scale::{
    hazmat::ArkScaleProjective,
    scale::{Decode, Encode},
};
use ark_std::marker::PhantomData;
use sp_ark_models::{
    bls12,
    short_weierstrass::{Affine, Projective, SWCurveConfig},
    CurveConfig,
};

pub type G2Affine<H> = bls12::G2Affine<crate::curves::Config<H>>;
pub type G2Projective<H> = bls12::G2Projective<crate::curves::Config<H>>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config<H: CurveHooks>(PhantomData<fn() -> H>);

impl<H: CurveHooks> CurveConfig for Config<H> {
    type BaseField = <ArkG2Config as CurveConfig>::BaseField;
    type ScalarField = <ArkG2Config as CurveConfig>::ScalarField;

    const COFACTOR: &'static [u64] = <ArkG2Config as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <ArkG2Config as CurveConfig>::COFACTOR_INV;
}

impl<H: CurveHooks> SWCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkG2Config as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkG2Config as SWCurveConfig>::COEFF_B;

    const GENERATOR: Affine<Self> = Affine::<Self>::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkG2Config as SWCurveConfig>::mul_by_a(elem)
    }

    // For any internal error returns zero.
    fn msm(
        bases: &[Affine<Self>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: ArkScale<&[Affine<Self>]> = bases.into();
        let scalars: ArkScale<&[<Self as CurveConfig>::ScalarField]> = scalars.into();

        let res = H::bls12_377_msm_g2(bases.encode(), scalars.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map_err(|_| 0).map(|res| res.0)
    }

    // For any internal error returns zero.
    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: ArkScaleProjective<Projective<Self>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let res =
            H::bls12_377_mul_projective_g2(base.encode(), scalar.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map(|v| v.0).unwrap_or_default()
    }

    // For any internal error returns zero.
    fn mul_affine(base: &Affine<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: Projective<Self> = (*base).into();
        let base: ArkScaleProjective<Projective<Self>> = base.into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let res =
            H::bls12_377_mul_projective_g2(base.encode(), scalar.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map(|v| v.0).unwrap_or_default()
    }
}

pub use ark_bls12_377::g2::{
    G2_GENERATOR_X, G2_GENERATOR_X_C0, G2_GENERATOR_X_C1, G2_GENERATOR_Y, G2_GENERATOR_Y_C0,
    G2_GENERATOR_Y_C1,
};

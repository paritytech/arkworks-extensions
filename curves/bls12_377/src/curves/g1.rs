use crate::{ArkScale, CurveHooks};
use ark_bls12_377::g1::Config as ArkG1Config;
use ark_scale::{
    hazmat::ArkScaleProjective,
    scale::{Decode, Encode},
};
use ark_std::marker::PhantomData;
use sp_ark_models::{
    bls12,
    short_weierstrass::{Affine as SWAffine, Projective, SWCurveConfig},
    twisted_edwards::{
        Affine as TEAffine, MontCurveConfig, Projective as TEProjective, TECurveConfig,
    },
    CurveConfig,
};

pub use ark_bls12_377::g1::{G1_GENERATOR_X, G1_GENERATOR_Y, TE_GENERATOR_X, TE_GENERATOR_Y};

pub type G1Affine<H> = bls12::G1Affine<crate::curves::Config<H>>;
pub type G1Projective<H> = bls12::G1Projective<crate::curves::Config<H>>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config<H: CurveHooks>(PhantomData<fn() -> H>);

impl<H: CurveHooks> CurveConfig for Config<H> {
    type BaseField = <ArkG1Config as CurveConfig>::BaseField;
    type ScalarField = <ArkG1Config as CurveConfig>::ScalarField;

    const COFACTOR: &'static [u64] = <ArkG1Config as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <ArkG1Config as CurveConfig>::COFACTOR_INV;
}

pub type G1SWAffine<H> = SWAffine<Config<H>>;
pub type G1TEAffine<H> = TEAffine<Config<H>>;
pub type G1TEProjective<H> = TEProjective<Config<H>>;

impl<H: CurveHooks> SWCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkG1Config as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkG1Config as SWCurveConfig>::COEFF_B;

    const GENERATOR: G1SWAffine<H> = G1SWAffine::<H>::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkG1Config as SWCurveConfig>::mul_by_a(elem)
    }

    // For any internal error returns zero.
    fn msm(
        bases: &[SWAffine<Self>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: ArkScale<&[SWAffine<Self>]> = bases.into();
        let scalars: ArkScale<&[<Self as CurveConfig>::ScalarField]> = scalars.into();

        let res = H::bls12_377_msm_g1(bases.encode(), scalars.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map_err(|_| 0).map(|res| res.0)
    }

    // For any internal error returns zero.
    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: ArkScaleProjective<Projective<Self>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let res =
            H::bls12_377_mul_projective_g1(base.encode(), scalar.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map(|v| v.0).unwrap_or_default()
    }

    // For any internal error returns zero.
    fn mul_affine(base: &SWAffine<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: Projective<Self> = (*base).into();
        let base: ArkScaleProjective<Projective<Self>> = base.into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let res =
            H::bls12_377_mul_projective_g1(base.encode(), scalar.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map(|v| v.0).unwrap_or_default()
    }
}

impl<H: CurveHooks> TECurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkG1Config as TECurveConfig>::COEFF_A;
    const COEFF_D: Self::BaseField = <ArkG1Config as TECurveConfig>::COEFF_D;

    const GENERATOR: G1TEAffine<H> = G1TEAffine::<H>::new_unchecked(TE_GENERATOR_X, TE_GENERATOR_Y);

    type MontCurveConfig = Config<H>;

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkG1Config as TECurveConfig>::mul_by_a(elem)
    }
}

impl<H: CurveHooks> MontCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkG1Config as MontCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkG1Config as MontCurveConfig>::COEFF_B;

    type TECurveConfig = Config<H>;
}

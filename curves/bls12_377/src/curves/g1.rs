use crate::{ArkScale, CurveHooks};

use ark_bls12_377::g1::Config as ArkConfig;
use ark_models_ext::{
    bls12,
    short_weierstrass::{Affine as SWAffine, Projective, SWCurveConfig},
    twisted_edwards::{
        Affine as TEAffine, MontCurveConfig, Projective as TEProjective, TECurveConfig,
    },
    CurveConfig,
};
use ark_scale::{
    hazmat::ArkScaleProjective,
    scale::{Decode, Encode},
};
use ark_std::marker::PhantomData;

pub use ark_bls12_377::g1::{G1_GENERATOR_X, G1_GENERATOR_Y, TE_GENERATOR_X, TE_GENERATOR_Y};

pub type G1Affine<H> = bls12::G1Affine<crate::Config<H>>;
pub type G1Projective<H> = bls12::G1Projective<crate::Config<H>>;

#[derive(Clone, Copy)]
pub struct Config<H: CurveHooks>(PhantomData<fn() -> H>);

impl<H: CurveHooks> CurveConfig for Config<H> {
    type BaseField = <ArkConfig as CurveConfig>::BaseField;
    type ScalarField = <ArkConfig as CurveConfig>::ScalarField;

    const COFACTOR: &'static [u64] = <ArkConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <ArkConfig as CurveConfig>::COFACTOR_INV;
}

pub type G1SWAffine<H> = SWAffine<Config<H>>;
pub type G1TEAffine<H> = TEAffine<Config<H>>;
pub type G1TEProjective<H> = TEProjective<Config<H>>;

impl<H: CurveHooks> SWCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_B;

    const GENERATOR: SWAffine<Self> =
        SWAffine::<Self>::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as SWCurveConfig>::mul_by_a(elem)
    }

    /// Multi scalar multiplication jumping into the user-defined `msm_g1` hook.
    ///
    /// On any internal error returns `Err(0)`.
    fn msm(
        bases: &[SWAffine<Self>],
        scalars: &[Self::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: ArkScale<&[SWAffine<Self>]> = bases.into();
        let scalars: ArkScale<&[Self::ScalarField]> = scalars.into();

        let res = H::bls12_377_msm_g1(bases.encode(), scalars.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map(|res| res.0).map_err(|_| 0)
    }

    /// Projective multiplication jumping into the user-defined `mul_projective_g1` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: ArkScaleProjective<Projective<Self>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let res =
            H::bls12_377_mul_projective_g1(base.encode(), scalar.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map(|v| v.0).unwrap_or_default()
    }

    /// Affine multiplication jumping into the user-defined `mul_projective_g1` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    fn mul_affine(base: &SWAffine<Self>, scalar: &[u64]) -> Projective<Self> {
        <Self as SWCurveConfig>::mul_projective(&(*base).into(), scalar)
    }
}

impl<H: CurveHooks> TECurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as TECurveConfig>::COEFF_A;
    const COEFF_D: Self::BaseField = <ArkConfig as TECurveConfig>::COEFF_D;

    const GENERATOR: G1TEAffine<H> = G1TEAffine::<H>::new_unchecked(TE_GENERATOR_X, TE_GENERATOR_Y);

    type MontCurveConfig = Config<H>;

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as TECurveConfig>::mul_by_a(elem)
    }
}

impl<H: CurveHooks> MontCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_B;

    type TECurveConfig = Config<H>;
}

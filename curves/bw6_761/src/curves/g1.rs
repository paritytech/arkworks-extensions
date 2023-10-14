use crate::{ArkScale, CurveHooks};

use ark_bw6_761::g1::Config as ArkConfig;
use ark_scale::{
    hazmat::ArkScaleProjective,
    scale::{Decode, Encode},
};
use ark_std::marker::PhantomData;
use sp_ark_models::{
    bw6,
    short_weierstrass::{Affine, Projective},
    {short_weierstrass::SWCurveConfig, CurveConfig},
};

pub use ark_bw6_761::g1::{G1_GENERATOR_X, G1_GENERATOR_Y};

pub type G1Affine<H> = bw6::G1Affine<crate::Config<H>>;
pub type G1Projective<H> = bw6::G1Projective<crate::Config<H>>;

#[derive(Clone, Default, PartialEq, Eq)]

pub struct Config<H: CurveHooks>(PhantomData<fn() -> H>);

impl<H: CurveHooks> CurveConfig for Config<H> {
    type BaseField = <ArkConfig as CurveConfig>::BaseField;
    type ScalarField = <ArkConfig as CurveConfig>::ScalarField;

    const COFACTOR: &'static [u64] = <ArkConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <ArkConfig as CurveConfig>::COFACTOR_INV;
}

impl<H: CurveHooks> SWCurveConfig for Config<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_B;

    const GENERATOR: Affine<Self> = Affine::<Self>::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as SWCurveConfig>::mul_by_a(elem)
    }

    /// Multi scalar multiplication jumping into the user-defined `msm_g1` hook.
    ///
    /// On any internal error returns `Err(0)`.
    fn msm(
        bases: &[Affine<Self>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: ArkScale<&[Affine<Self>]> = bases.into();
        let scalars: ArkScale<&[<Self as CurveConfig>::ScalarField]> = scalars.into();

        let res = H::bw6_761_msm_g1(bases.encode(), scalars.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map_err(|_| 0).map(|res| res.0)
    }

    /// Projective multiplication jumping into the user-defined `mul_projective_g1` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: ArkScaleProjective<Projective<Self>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let res = H::bw6_761_mul_projective_g1(base.encode(), scalar.encode()).unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map(|v| v.0).unwrap_or_default()
    }

    /// Affine multiplication jumping into the user-defined `mul_projective_g1` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    fn mul_affine(base: &Affine<Self>, scalar: &[u64]) -> Projective<Self> {
        <Self as SWCurveConfig>::mul_projective(&(*base).into(), scalar)
    }
}

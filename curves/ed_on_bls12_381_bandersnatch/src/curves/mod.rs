use crate::ArkScale;

use ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig as ArkConfig;
use ark_ff::MontFp;
use ark_scale::{
    hazmat::ArkScaleProjective,
    scale::{Decode, Encode},
};
use ark_std::marker::PhantomData;
use ark_std::vec::Vec;
use sp_ark_models::{
    models::CurveConfig,
    short_weierstrass::{self, SWCurveConfig},
    twisted_edwards::{self, Affine, MontCurveConfig, Projective, TECurveConfig},
};

#[cfg(test)]
mod tests;

// TODO: @davxy
// Directly use upstream generator values as soon as version > 0.4.0 is released.
// Ref: https://github.com/arkworks-rs/curves/pull/184

/// x coordinate for TE curve generator
pub const TE_GENERATOR_X: <ArkConfig as CurveConfig>::BaseField =
    MontFp!("18886178867200960497001835917649091219057080094937609519140440539760939937304");

/// y coordinate for TE curve generator
pub const TE_GENERATOR_Y: <ArkConfig as CurveConfig>::BaseField =
    MontFp!("19188667384257783945677642223292697773471335439753913231509108946878080696678");

/// x coordinate for SW curve generator
pub const SW_GENERATOR_X: <ArkConfig as CurveConfig>::BaseField =
    MontFp!("30900340493481298850216505686589334086208278925799850409469406976849338430199");

/// y coordinate for SW curve generator
pub const SW_GENERATOR_Y: <ArkConfig as CurveConfig>::BaseField =
    MontFp!("12663882780877899054958035777720958383845500985908634476792678820121468453298");

pub type EdwardsAffine<H> = twisted_edwards::Affine<BandersnatchConfig<H>>;
pub type EdwardsProjective<H> = twisted_edwards::Projective<BandersnatchConfig<H>>;

pub type SWAffine<H> = short_weierstrass::Affine<BandersnatchConfig<H>>;
pub type SWProjective<H> = short_weierstrass::Projective<BandersnatchConfig<H>>;

#[derive(Clone, Copy)]
pub struct BandersnatchConfig<H: CurveHooks>(PhantomData<fn() -> H>);

pub type EdwardsConfig<H> = BandersnatchConfig<H>;
pub type SWConfig<H> = BandersnatchConfig<H>;

pub trait CurveHooks: 'static {
    fn ed_on_bls12_381_bandersnatch_te_msm(bases: Vec<u8>, scalars: Vec<u8>)
        -> Result<Vec<u8>, ()>;
    fn ed_on_bls12_381_bandersnatch_sw_msm(bases: Vec<u8>, scalars: Vec<u8>)
        -> Result<Vec<u8>, ()>;
    fn ed_on_bls12_381_bandersnatch_sw_mul_projective(
        base: Vec<u8>,
        scalar: Vec<u8>,
    ) -> Result<Vec<u8>, ()>;
    fn ed_on_bls12_381_bandersnatch_te_mul_projective(
        base: Vec<u8>,
        scalar: Vec<u8>,
    ) -> Result<Vec<u8>, ()>;
}

impl<H: CurveHooks> CurveConfig for BandersnatchConfig<H> {
    const COFACTOR: &'static [u64] = <ArkConfig as CurveConfig>::COFACTOR;
    const COFACTOR_INV: Self::ScalarField = <ArkConfig as CurveConfig>::COFACTOR_INV;

    type BaseField = <ArkConfig as CurveConfig>::BaseField;
    type ScalarField = <ArkConfig as CurveConfig>::ScalarField;
}

impl<H: CurveHooks> TECurveConfig for BandersnatchConfig<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as TECurveConfig>::COEFF_A;
    const COEFF_D: Self::BaseField = <ArkConfig as TECurveConfig>::COEFF_D;

    const GENERATOR: Affine<Self> = Affine::<Self>::new_unchecked(TE_GENERATOR_X, TE_GENERATOR_Y);

    type MontCurveConfig = BandersnatchConfig<H>;

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as TECurveConfig>::mul_by_a(elem)
    }

    /// Multi scalar multiplication jumping into the user-defined `te_msm` hook.
    ///
    /// On any internal error returns `Err(0)`.
    fn msm(
        bases: &[Affine<Self>],
        scalars: &[Self::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: ArkScale<&[Affine<Self>]> = bases.into();
        let scalars: ArkScale<&[Self::ScalarField]> = scalars.into();

        let res = H::ed_on_bls12_381_bandersnatch_te_msm(bases.encode(), scalars.encode())
            .unwrap_or_default();

        let res =
            ArkScaleProjective::<Projective<BandersnatchConfig<H>>>::decode(&mut res.as_slice());
        res.map(|res| res.0).map_err(|_| 0)
    }

    /// Projective multiplication jumping into the user-defined `te_mul_projective` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: ArkScaleProjective<Projective<Self>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let res = H::ed_on_bls12_381_bandersnatch_te_mul_projective(base.encode(), scalar.encode())
            .unwrap_or_default();

        let res = ArkScaleProjective::<Projective<Self>>::decode(&mut res.as_slice());
        res.map(|v| v.0).unwrap_or_default()
    }

    /// Affine multiplication jumping into the user-defined `te_mul_projective` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    fn mul_affine(base: &Affine<Self>, scalar: &[u64]) -> Projective<Self> {
        <Self as TECurveConfig>::mul_projective(&(*base).into(), scalar)
    }
}

impl<H: CurveHooks> MontCurveConfig for BandersnatchConfig<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_B;

    type TECurveConfig = BandersnatchConfig<H>;
}

impl<H: CurveHooks> SWCurveConfig for BandersnatchConfig<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_B;

    const GENERATOR: short_weierstrass::Affine<Self> =
        short_weierstrass::Affine::<Self>::new_unchecked(SW_GENERATOR_X, SW_GENERATOR_Y);

    /// Multi scalar multiplication jumping into the user-defined `sw_msm` hook.
    ///
    /// On any internal error returns `Err(0)`.
    fn msm(
        bases: &[short_weierstrass::Affine<Self>],
        scalars: &[Self::ScalarField],
    ) -> Result<short_weierstrass::Projective<Self>, usize> {
        let bases: ArkScale<&[short_weierstrass::Affine<Self>]> = bases.into();
        let scalars: ArkScale<&[Self::ScalarField]> = scalars.into();

        let res = H::ed_on_bls12_381_bandersnatch_sw_msm(bases.encode(), scalars.encode())
            .unwrap_or_default();

        let res =
            ArkScaleProjective::<short_weierstrass::Projective<Self>>::decode(&mut res.as_slice());
        res.map(|res| res.0).map_err(|_| 0)
    }

    /// Projective multiplication jumping into the user-defined `sw_mul_projective` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    fn mul_projective(
        base: &short_weierstrass::Projective<Self>,
        scalar: &[u64],
    ) -> short_weierstrass::Projective<Self> {
        let base: ArkScaleProjective<short_weierstrass::Projective<Self>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let res = H::ed_on_bls12_381_bandersnatch_sw_mul_projective(base.encode(), scalar.encode())
            .unwrap_or_default();

        let res =
            ArkScaleProjective::<short_weierstrass::Projective<Self>>::decode(&mut res.as_slice());
        res.map(|v| v.0).unwrap_or_default()
    }

    /// Affine multiplication jumping into the user-defined `sw_mul_projective` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    fn mul_affine(
        base: &short_weierstrass::Affine<Self>,
        scalar: &[u64],
    ) -> short_weierstrass::Projective<Self> {
        <Self as SWCurveConfig>::mul_projective(&(*base).into(), scalar)
    }
}

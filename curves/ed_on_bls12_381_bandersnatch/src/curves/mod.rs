use ark_ed_on_bls12_381_bandersnatch::BandersnatchConfig as ArkConfig;
use ark_ff::MontFp;
use ark_models_ext::{
    models::CurveConfig,
    short_weierstrass::{self, SWCurveConfig},
    twisted_edwards::{self, MontCurveConfig, TECurveConfig},
};
use ark_std::marker::PhantomData;

#[cfg(test)]
mod tests;

// TODO: use upstream generator values as soon as version > 0.4.0 is released.
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

/// Hooks for *Ed-on-BLS12-377-Bandernatch*.
pub trait CurveHooks: 'static + Sized {
    /// Twisted Edwards multi scalar multiplication.
    fn ed_on_bls12_381_bandersnatch_te_msm(
        bases: &[EdwardsAffine<Self>],
        scalars: &[<EdwardsConfig<Self> as CurveConfig>::ScalarField],
    ) -> Result<EdwardsProjective<Self>, ()>;

    /// Twisted Edwards projective multiplication.
    fn ed_on_bls12_381_bandersnatch_te_mul_projective(
        base: &EdwardsProjective<Self>,
        scalar: &[u64],
    ) -> Result<EdwardsProjective<Self>, ()>;

    /// Short Weierstrass multi scalar multiplication.
    fn ed_on_bls12_381_bandersnatch_sw_msm(
        bases: &[SWAffine<Self>],
        scalars: &[<SWConfig<Self> as CurveConfig>::ScalarField],
    ) -> Result<SWProjective<Self>, ()>;

    /// Short Weierstrass projective multiplication.
    fn ed_on_bls12_381_bandersnatch_sw_mul_projective(
        base: &SWProjective<Self>,
        scalar: &[u64],
    ) -> Result<SWProjective<Self>, ()>;
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

    const GENERATOR: EdwardsAffine<H> =
        EdwardsAffine::<H>::new_unchecked(TE_GENERATOR_X, TE_GENERATOR_Y);

    type MontCurveConfig = Self;

    /// Multi scalar multiplication jumping into the user-defined `te_msm` hook.
    ///
    /// On any internal error returns `Err(0)`.
    #[inline(always)]
    fn msm(
        bases: &[EdwardsAffine<H>],
        scalars: &[Self::ScalarField],
    ) -> Result<EdwardsProjective<H>, usize> {
        H::ed_on_bls12_381_bandersnatch_te_msm(bases, scalars).map_err(|_| 0)
    }

    /// Projective multiplication jumping into the user-defined `te_mul_projective` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_projective(base: &EdwardsProjective<H>, scalar: &[u64]) -> EdwardsProjective<H> {
        H::ed_on_bls12_381_bandersnatch_te_mul_projective(base, scalar).unwrap_or_default()
    }

    /// Affine multiplication jumping into the user-defined `te_mul_projective` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_affine(base: &EdwardsAffine<H>, scalar: &[u64]) -> EdwardsProjective<H> {
        <Self as TECurveConfig>::mul_projective(&(*base).into(), scalar)
    }

    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        <ArkConfig as TECurveConfig>::mul_by_a(elem)
    }
}

impl<H: CurveHooks> SWCurveConfig for BandersnatchConfig<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as SWCurveConfig>::COEFF_B;

    const GENERATOR: SWAffine<H> = SWAffine::<H>::new_unchecked(SW_GENERATOR_X, SW_GENERATOR_Y);

    /// Multi scalar multiplication jumping into the user-defined `sw_msm` hook.
    ///
    /// On any internal error returns `Err(0)`.
    #[inline(always)]
    fn msm(bases: &[SWAffine<H>], scalars: &[Self::ScalarField]) -> Result<SWProjective<H>, usize> {
        H::ed_on_bls12_381_bandersnatch_sw_msm(bases, scalars).map_err(|_| 0)
    }

    /// Projective multiplication jumping into the user-defined `sw_mul_projective` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_projective(base: &SWProjective<H>, scalar: &[u64]) -> SWProjective<H> {
        H::ed_on_bls12_381_bandersnatch_sw_mul_projective(base, scalar).unwrap_or_default()
    }

    /// Affine multiplication jumping into the user-defined `sw_mul_projective` hook.
    ///
    /// On any internal error returns `Projective::zero()`.
    #[inline(always)]
    fn mul_affine(base: &SWAffine<H>, scalar: &[u64]) -> SWProjective<H> {
        <Self as SWCurveConfig>::mul_projective(&(*base).into(), scalar)
    }
}

impl<H: CurveHooks> MontCurveConfig for BandersnatchConfig<H> {
    const COEFF_A: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_A;
    const COEFF_B: Self::BaseField = <ArkConfig as MontCurveConfig>::COEFF_B;

    type TECurveConfig = Self;
}

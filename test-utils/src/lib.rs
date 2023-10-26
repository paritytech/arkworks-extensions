//! Implementations for test hooks.
//!
//! We just safely transmute from Arkworks-Ext types to Arkworks upstream types by
//! encoding and deconding and jump into the *Arkworks* upstream methods.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::result_unit_err)]

use ark_ec::{
    pairing::{MillerLoopOutput, Pairing},
    short_weierstrass::{Affine as SWAffine, Projective as SWProjective, SWCurveConfig},
    twisted_edwards::{Affine as TEAffine, Projective as TEProjective, TECurveConfig},
    CurveConfig, VariableBaseMSM,
};
use ark_scale::{
    ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate},
    scale::{Decode, Encode},
};
use ark_std::vec::Vec;

#[cfg(feature = "scale-no-compress")]
const SCALE_COMPRESS: Compress = Compress::No;
#[cfg(not(feature = "scale-no-compress"))]
const SCALE_COMPRESS: Compress = Compress::Yes;

/// SCALE codec usage settings.
///
/// Determines whether compression and validation has been enabled for SCALE codec
/// with respect to ARK related types.
///
/// WARNING: usage of validation can be dangeruos in the hooks as it may re-enter
/// the same hook ad cause a stack-overflow.
const SCALE_USAGE: u8 = ark_scale::make_usage(SCALE_COMPRESS, Validate::No);

type ArkScale<T> = ark_scale::ArkScale<T, SCALE_USAGE>;

trait TryTransmute {
    fn try_transmute<U: CanonicalDeserialize>(self) -> Result<U, ()>;
}

impl<T: CanonicalSerialize> TryTransmute for T {
    fn try_transmute<U: CanonicalDeserialize>(self) -> Result<U, ()> {
        let buf = ArkScale::from(self).encode();
        ArkScale::<U>::decode(&mut &buf[..])
            .map(|v| v.0)
            .map_err(|_| ())
    }
}

pub fn multi_miller_loop_generic<ExtPairing: Pairing, ArkPairing: Pairing>(
    g1: impl Iterator<Item = ExtPairing::G1Prepared>,
    g2: impl Iterator<Item = ExtPairing::G2Prepared>,
) -> Result<ExtPairing::TargetField, ()> {
    let g1: Vec<ArkPairing::G1Affine> = g1.collect::<Vec<_>>().try_transmute()?;
    let g2: Vec<ArkPairing::G2Affine> = g2.collect::<Vec<_>>().try_transmute()?;

    let res = ArkPairing::multi_miller_loop(g1, g2).0;
    res.try_transmute()
}

pub fn final_exponentiation_generic<ExtPairing: Pairing, ArkPairing: Pairing>(
    target: ExtPairing::TargetField,
) -> Result<ExtPairing::TargetField, ()> {
    let target: ArkPairing::TargetField = target.try_transmute()?;

    let res = ArkPairing::final_exponentiation(MillerLoopOutput(target)).ok_or(())?;
    res.try_transmute()
}

pub fn msm_sw_generic<ExtCurve: SWCurveConfig, ArkCurve: SWCurveConfig>(
    bases: &[SWAffine<ExtCurve>],
    scalars: &[ExtCurve::ScalarField],
) -> Result<SWProjective<ExtCurve>, ()> {
    let bases: Vec<SWAffine<ArkCurve>> = bases.try_transmute()?;
    let scalars: Vec<ArkCurve::ScalarField> = scalars.try_transmute()?;

    let res = <SWProjective<ArkCurve> as VariableBaseMSM>::msm(&bases, &scalars).map_err(|_| ())?;
    res.try_transmute()
}

pub fn msm_te_generic<ExtConfig: TECurveConfig, ArkConfig: TECurveConfig>(
    bases: &[TEAffine<ExtConfig>],
    scalars: &[ExtConfig::ScalarField],
) -> Result<TEProjective<ExtConfig>, ()> {
    let bases: Vec<TEAffine<ArkConfig>> = bases.try_transmute()?;
    let scalars: Vec<<ArkConfig as CurveConfig>::ScalarField> = scalars.try_transmute()?;

    let res =
        <TEProjective<ArkConfig> as VariableBaseMSM>::msm(&bases, &scalars).map_err(|_| ())?;
    res.try_transmute()
}

pub fn mul_projective_sw_generic<ExtConfig: SWCurveConfig, ArkConfig: SWCurveConfig>(
    base: &SWProjective<ExtConfig>,
    scalar: &[u64],
) -> Result<SWProjective<ExtConfig>, ()> {
    let base: SWProjective<ArkConfig> = base.try_transmute()?;

    let res = <ArkConfig as SWCurveConfig>::mul_projective(&base, scalar);
    res.try_transmute()
}

pub fn mul_projective_te_generic<ExtConfig: TECurveConfig, ArkConfig: TECurveConfig>(
    base: &TEProjective<ExtConfig>,
    scalar: &[u64],
) -> Result<TEProjective<ExtConfig>, ()> {
    let base: TEProjective<ArkConfig> = base.try_transmute()?;

    let res = <ArkConfig as TECurveConfig>::mul_projective(&base, scalar);
    res.try_transmute()
}

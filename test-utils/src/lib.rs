//! Implementations for test hooks.
//!
//! We just safely transmute from Arkworks-Ext types to Arkworks upstream types by
//! encoding and deconding and jump into the *Arkworks* upstream methods.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::result_unit_err)]

use ark_ec::{
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
    short_weierstrass::{self, Affine as SWAffine, Projective as SWProjective, SWCurveConfig},
    twisted_edwards::{self, Affine as TEAffine, Projective as TEProjective, TECurveConfig},
    CurveConfig, VariableBaseMSM,
};
use ark_scale::{
    ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Compress, Validate},
    hazmat::ArkScaleProjective,
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
pub const SCALE_USAGE: u8 = ark_scale::make_usage(SCALE_COMPRESS, Validate::No);

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

pub fn multi_miller_loop_generic<Curve: Pairing>(g1: Vec<u8>, g2: Vec<u8>) -> Result<Vec<u8>, ()> {
    let g1 = <ArkScale<Vec<<Curve as Pairing>::G1Affine>> as Decode>::decode(&mut g1.as_slice())
        .map_err(|_| ())?;
    let g2 = <ArkScale<Vec<<Curve as Pairing>::G2Affine>> as Decode>::decode(&mut g2.as_slice())
        .map_err(|_| ())?;

    let result = Curve::multi_miller_loop(g1.0, g2.0).0;

    let result: ArkScale<<Curve as Pairing>::TargetField> = result.into();
    Ok(result.encode())
}

pub fn multi_miller_loop_generic2<ExtCurve: Pairing, ArkCurve: Pairing>(
    g1: impl Iterator<Item = ExtCurve::G1Prepared>,
    g2: impl Iterator<Item = ExtCurve::G2Prepared>,
) -> Result<ExtCurve::TargetField, ()> {
    let g1: ArkScale<Vec<ExtCurve::G1Prepared>> = g1.collect::<Vec<_>>().into();
    let buf = g1.encode();
    let g1 = ArkScale::<Vec<ArkCurve::G1Affine>>::decode(&mut buf.as_slice()).map_err(|_| ())?;

    let g2: ArkScale<Vec<ExtCurve::G2Prepared>> = g2.collect::<Vec<_>>().into();
    let buf = g2.encode();
    let g2 = ArkScale::<Vec<ArkCurve::G2Affine>>::decode(&mut buf.as_slice()).map_err(|_| ())?;

    let res: ArkScale<ArkCurve::TargetField> = ArkCurve::multi_miller_loop(g1.0, g2.0).0.into();
    let buf = res.encode();
    let res = ArkScale::<ExtCurve::TargetField>::decode(&mut buf.as_slice()).map_err(|_| ())?;

    Ok(res.0)
}

pub fn final_exponentiation_generic<Curve: Pairing>(target: Vec<u8>) -> Result<Vec<u8>, ()> {
    let target =
        <ArkScale<<Curve as Pairing>::TargetField> as Decode>::decode(&mut target.as_slice())
            .map_err(|_| ())?;

    let result = Curve::final_exponentiation(MillerLoopOutput(target.0)).ok_or(())?;

    let result: ArkScale<PairingOutput<Curve>> = result.into();
    Ok(result.encode())
}

pub fn final_exponentiation_generic2<ExtCurve: Pairing, ArkCurve: Pairing>(
    target: ExtCurve::TargetField,
) -> Result<ExtCurve::TargetField, ()> {
    let target: ArkCurve::TargetField = target.try_transmute()?;

    let res = ArkCurve::final_exponentiation(MillerLoopOutput(target)).ok_or(())?;
    res.try_transmute()
}

pub fn msm_sw_generic<Curve: SWCurveConfig>(
    bases: Vec<u8>,
    scalars: Vec<u8>,
) -> Result<Vec<u8>, ()> {
    let bases =
        <ArkScale<Vec<short_weierstrass::Affine<Curve>>> as Decode>::decode(&mut bases.as_slice())
            .map_err(|_| ())?;
    let scalars = <ArkScale<Vec<<Curve as CurveConfig>::ScalarField>> as Decode>::decode(
        &mut scalars.as_slice(),
    )
    .map_err(|_| ())?;

    let result =
        <short_weierstrass::Projective<Curve> as VariableBaseMSM>::msm(&bases.0, &scalars.0)
            .map_err(|_| ())?;

    let result: ArkScaleProjective<short_weierstrass::Projective<Curve>> = result.into();
    Ok(result.encode())
}

pub fn msm_sw_generic2<ExtCurve: SWCurveConfig, ArkCurve: SWCurveConfig>(
    bases: &[SWAffine<ExtCurve>],
    scalars: &[ExtCurve::ScalarField],
) -> Result<short_weierstrass::Projective<ExtCurve>, ()> {
    let bases: Vec<SWAffine<ArkCurve>> = bases.try_transmute()?;
    let scalars: Vec<ArkCurve::ScalarField> = scalars.try_transmute()?;

    let res = <SWProjective<ArkCurve> as VariableBaseMSM>::msm(&bases, &scalars).map_err(|_| ())?;
    res.try_transmute()
}

pub fn msm_te_generic<Curve: TECurveConfig>(
    bases: Vec<u8>,
    scalars: Vec<u8>,
) -> Result<Vec<u8>, ()> {
    let bases =
        <ArkScale<Vec<twisted_edwards::Affine<Curve>>> as Decode>::decode(&mut bases.as_slice())
            .map_err(|_| ())?;
    let scalars = <ArkScale<Vec<<Curve as CurveConfig>::ScalarField>> as Decode>::decode(
        &mut scalars.as_slice(),
    )
    .map_err(|_| ())?;

    let result = <twisted_edwards::Projective<Curve> as VariableBaseMSM>::msm(&bases.0, &scalars.0)
        .map_err(|_| ())?;

    let result: ArkScaleProjective<twisted_edwards::Projective<Curve>> = result.into();
    Ok(result.encode())
}

pub fn msm_te_generic2<ExtConfig: TECurveConfig, ArkConfig: TECurveConfig>(
    bases: &[TEAffine<ExtConfig>],
    scalars: &[ExtConfig::ScalarField],
) -> Result<TEProjective<ExtConfig>, ()> {
    let bases: Vec<TEAffine<ArkConfig>> = bases.try_transmute()?;
    let scalars: Vec<<ArkConfig as CurveConfig>::ScalarField> = scalars.try_transmute()?;

    let res =
        <TEProjective<ArkConfig> as VariableBaseMSM>::msm(&bases, &scalars).map_err(|_| ())?;
    res.try_transmute()
}

pub fn mul_projective_generic<Group: SWCurveConfig>(
    base: Vec<u8>,
    scalar: Vec<u8>,
) -> Result<Vec<u8>, ()> {
    let base = <ArkScaleProjective<short_weierstrass::Projective<Group>> as Decode>::decode(
        &mut base.as_slice(),
    )
    .map_err(|_| ())?;
    let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).map_err(|_| ())?;

    let result = <Group as SWCurveConfig>::mul_projective(&base.0, &scalar.0);

    let result: ArkScaleProjective<short_weierstrass::Projective<Group>> = result.into();
    Ok(result.encode())
}

pub fn mul_projective_generic2<ExtConfig: SWCurveConfig, ArkConfig: SWCurveConfig>(
    base: &SWProjective<ExtConfig>,
    scalar: &[u64],
) -> Result<SWProjective<ExtConfig>, ()> {
    let base: SWProjective<ArkConfig> = base.try_transmute()?;

    let res = <ArkConfig as SWCurveConfig>::mul_projective(&base, scalar);
    res.try_transmute()
}

pub fn mul_projective_te_generic<Group: TECurveConfig>(
    base: Vec<u8>,
    scalar: Vec<u8>,
) -> Result<Vec<u8>, ()> {
    let base = <ArkScaleProjective<twisted_edwards::Projective<Group>> as Decode>::decode(
        &mut base.as_slice(),
    )
    .map_err(|_| ())?;
    let scalar = <ArkScale<Vec<u64>> as Decode>::decode(&mut scalar.as_slice()).map_err(|_| ())?;

    let result = <Group as TECurveConfig>::mul_projective(&base.0, &scalar.0);

    let result: ArkScaleProjective<twisted_edwards::Projective<Group>> = result.into();
    Ok(result.encode())
}

pub fn mul_projective_te_generic2<ExtConfig: TECurveConfig, ArkConfig: TECurveConfig>(
    base: &TEProjective<ExtConfig>,
    scalar: &[u64],
) -> Result<TEProjective<ExtConfig>, ()> {
    let base: TEProjective<ArkConfig> = base.try_transmute()?;

    let res = <ArkConfig as TECurveConfig>::mul_projective(&base, scalar);
    res.try_transmute()
}

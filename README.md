# Arkworks Extensions

## Overview

This library extends [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra).

We fork the popular elliptic curves `BLS12_381`, `BLS12_377`, `BW6_761`,
`ED_ON_BLS12_381_BANDERSNATCH` and `ED_ON_BLS12_377` in a way which allows
delegating some of the most computationally expensive operations to some user
defined hooks.

We also provide forks of the models `BW6` and `BLS12` to avoid the point
preparation before the hooks calls during pairing operations. Therefore, we
redefine the elliptic curve sub-groups `G2` for both models as thin wrappers
around the affine points and move the point preparation procedure to the
user defined hook.

## Usage

The following usage example is extracted from the hooks provided by
[substrate-curves](https://github.com/paritytech/substrate-curves) project.

The project provides a set of ready to use `CurveHooks` implementations compatible with
[Substrate](https://github.com/paritytech/polkadot-sdk/primitives/crypto/ec-utils)
host functions to jump from *wasm32* computational domain into the native host.

The motivation is:
- native target is typically more efficient that *wasm32*.
- *wasm32* is single thread while in the native target we can hopefully leverage
  the Arkworks `parallel` feature.

Note that Substrate elliptic curves host functions take and return raw byte arrays
representing SCALE encoded values.

### BLS12-377

```rust
use ark_bls12_377_ext::CurveHooks;
use ark_ec::{pairing::Pairing, CurveConfig}
use ark_scale::{
    ark_serialize::{Compress, Validate},
    scale::{Decode, Encode},
};
use sp_crypto_ec_utils::bls12_377_ops;


const SCALE_USAGE: u8 = ark_scale::make_usage(Compress::No, Validate::No);
type ArkScale<T> = ark_scale::ArkScale<T, SCALE_USAGE>;o
type ArkScaleProjective<T> = ark_scale::hazmat::ArkScaleProjective<T>;

#[derive(Copy, Clone)]
pub struct HostHooks;

type Bls12_377 = ark_bls12_377_ext::Bls12_377<HostHooks>;
type G1Affine = ark_bls12_377_ext::g1::G1Affine<HostHooks>;
type G1Config = ark_bls12_377_ext::g1::Config<HostHooks>;
type G2Affine = ark_bls12_377_ext::g2::G2Affine<HostHooks>;
type G2Config = ark_bls12_377_ext::g2::Config<HostHooks>;


impl CurveHooks for HostHooks {
    fn bls12_377_multi_miller_loop(
        g1: impl Iterator<Item = <Bls12_377 as Pairing>::G1Prepared>,
        g2: impl Iterator<Item = <Bls12_377 as Pairing>::G2Prepared>,
    ) -> Result<<Bls12_377 as Pairing>::TargetField, ()> {
        // Encode to SCALE to call into Substrate HF
        let g1 = ArkScale::from(g1.collect::<Vec<_>>()).encode();
        let g2 = ArkScale::from(g2.collect::<Vec<_>>()).encode();
        // Call into native host function
        let res = bls12_377_ops::bls12_377_multi_miller_loop(g1, g2).unwrap_or_default();
        // Decode from SCALE
        let res = ArkScale::<<Bls12_377 as Pairing>::TargetField>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn bls12_377_final_exponentiation(
        target: <Bls12_377 as Pairing>::TargetField,
    ) -> Result<<Bls12_377 as Pairing>::TargetField, ()> {
        let target = ArkScale::from(target).encode();
        let res = bls12_377_ops::bls12_377_final_exponentiation(target).unwrap_or_default();
        let res = ArkScale::<<Bls12_377 as Pairing>::TargetField>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn bls12_377_msm_g1(
        bases: &[G1Affine],
        scalars: &[<G1Config as CurveConfig>::ScalarField],
    ) -> Result<G1Projective, ()> {
        let bases = ArkScale::from(bases).encode();
        let scalars = ArkScale::from(scalars).encode();
        let res = bls12_377_ops::bls12_377_msm_g1(bases, scalars).unwrap_or_default();
        let res = ArkScaleProjective::<G1Projective>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn bls12_377_msm_g2(
        bases: &[G2Affine],
        scalars: &[<G2Config as CurveConfig>::ScalarField],
    ) -> Result<G2Projective, ()> {
        let bases = ArkScale::from(bases).encode();
        let scalars = ArkScale::from(scalars).encode();
        let res = bls12_377_ops::bls12_377_msm_g2(bases, scalars).unwrap_or_default();
        let res = ArkScaleProjective::<G2Projective>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn bls12_377_mul_projective_g1(
        base: &G1Projective,
        scalar: &[u64],
    ) -> Result<G1Projective, ()> {
        let base = ArkScaleProjective::from(base).encode();
        let scalar = ArkScale::from(scalar).encode();
        let res = bls12_377_ops::bls12_377_mul_projective_g1(base, scalar).unwrap_or_default();
        let res = ArkScaleProjective::<G1Projective>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }

    fn bls12_377_mul_projective_g2(
        base: &G2Projective,
        scalar: &[u64],
    ) -> Result<G2Projective, ()> {
        let base = ArkScaleProjective::from(base).encode();
        let scalar = ArkScale::from(scalar).encode();
        let res = bls12_377_ops::bls12_377_mul_projective_g2(base, scalar).unwrap_or_default();
        let res = ArkScaleProjective::<G2Projective>::decode(&mut res.as_slice());
        res.map(|v| v.0).map_err(|_| ())
    }o
}
```

For more working examples refer to [Ark Substrate](https://github.com/davxy/ark-substrate-examples).


## ⚠️ Known Limitations ⚠️

Be aware that, while in the hook context, any usage of functions which may
re-enter into the same hook with the same value, may cause an infinite loop.

We are aware of hooks re-entrancy issues when using **point checked
deserialization** in projective multiplication hooks.

In particular, if you serialize and deserialize (**with point checking**) the
input point in one of the projective multiplication hooks then we end up
re-entering the multiplication hook with the same value as a consequence of the
internally performed check.

The following invocation flow applies:

1. [Validation of deserialized value](https://github.com/arkworks-rs/algebra/blob/c0666a81190dbcade1b735ffd383a5f577dd33d5/ec/src/models/twisted_edwards/mod.rs#L145-L147).
2. [Check if point is in the correct subgroup](https://github.com/arkworks-rs/algebra/blob/c0666a81190dbcade1b735ffd383a5f577dd33d5/ec/src/models/twisted_edwards/affine.rs#L321).
3. [Jump into the `TECurveConfig` for the check](https://github.com/arkworks-rs/algebra/blob/c0666a81190dbcade1b735ffd383a5f577dd33d5/ec/src/models/twisted_edwards/affine.rs#L159).
4. Calls the "custom" (defined by this crate) implementation of `mul_affine` which calls `mul_projective`.
5. Goto 1

So pay special attention to the actions in your `CurveHooks` implementations.

If you encounter any other way to trigger the open, please file an issue.

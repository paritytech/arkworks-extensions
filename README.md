# Arkworks Extensions

## Overview

This is a partial fork of the code from
[arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) and
[arkworks-rs/curves](https://github.com/arkworks-rs/curves).

We fork the popular elliptic curves `BLS12_381`, `BLS12_377`, `BW6_761`,
`ED_ON_BLS12_381_BANDERSNATCH` and `ED_ON_BLS12_377` in a way which allows us
to replace the elliptic curve arithmetic which is computationally heavy
with user defined hooks (e.g. to jump into the host from a wasm context).

We also provide forks of the models `BW6` and `BLS12` to avoid the point
preparation before the hooks calls during pairing operations. Therefore we
redefine the elliptic curve sub-groups `G2` for both models as thin wrappers
around the affine points and move the point preparation procedure to the
user defined hook.

## Usage

The following usage examples are extracted from the hooks provided by
[Substrate](https://github.com/paritytech/polkadot-sdk/primitives/crypto/ec-utils)
which are used to jump from wasm32 into the native host.

For working examples refer to Ark Substrate examples repo
[here](https://github.com/paritytech/ark-substrate/examples).

### BLS12-377

```rust
pub struct Host;

impl ark_bls12_377_ext::CurveHooks for Host {
    fn bls12_377_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bls12_377_multi_miller_loop(a, b)
    }
    fn bls12_377_final_exponentiation(f: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bls12_377_final_exponentiation(f)
    }
    fn bls12_377_msm_g1(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bls12_377_msm_g1(bases, scalars)
    }
    fn bls12_377_msm_g2(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bls12_377_msm_g2(bases, scalars)
    }
    fn bls12_377_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bls12_377_mul_projective_g1(base, scalar)
    }
    fn bls12_377_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bls12_377_mul_projective_g2(base, scalar)
    }
}

type Bls12_377 = ark_bls12_377_ext::Bls12_377<Host>;
```

### BLS12-381

```rust
pub struct Host;

impl ark_bls12_381_ext::CurveHooks for Host {
    fn bls12_381_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bls12_381_multi_miller_loop(a, b)
    }
    fn bls12_381_final_exponentiation(f: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bls12_381_final_exponentiation(f)
    }
    fn bls12_381_msm_g1(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bls12_381_msm_g1(bases, scalars)
    }
    fn bls12_381_msm_g2(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bls12_381_msm_g2(bases, scalars)
    }
    fn bls12_381_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bls12_381_mul_projective_g1(base, scalar)
    }
    fn bls12_381_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bls12_381_mul_projective_g2(base, scalar)
    }
}

type Bls12_381 = ark_bls12_381_ext::Bls12_381<Host>;
```

### BW6-761

```rust
pub struct Host;

impl ark_bw6_761_ext::CurveHooks for Host {
    fn bw6_761_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bw6_761_multi_miller_loop(a, b)
    }
    fn bw6_761_final_exponentiation(f12: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bw6_761_final_exponentiation(f12)
    }
    fn bw6_761_msm_g1(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bw6_761_msm_g1(bases, scalars)
    }
    fn bw6_761_msm_g2(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bw6_761_msm_g2(bases, scalars)
    }
    fn bw6_761_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bw6_761_mul_projective_g1(base, scalar)
    }
    fn bw6_761_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::bw6_761_mul_projective_g2(base, scalar)
    }
}

type BW6_761 = ark_bw6_761_ext::BW6_761<Host>;
```

### ED-ON-BLS12-377

```rust
pub struct Host;

impl ark_ed_on_bls12_377_ext::CurveHooks for Host {
    fn ed_on_bls12_377_msm(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::ed_on_bls12_377_msm(bases, scalars)
    }
    fn ed_on_bls12_377_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::ed_on_bls12_377_mul_projective(base, scalar)
    }
}

type EdwardsProjective = ark_ed_on_bls12_377_ext::EdwardsProjective<Host>;
```

### ED-ON-BLS12-381-BANDERSNATCH

```rust
pub struct Host;

impl ark_ed_on_bls12_381_bandersnatch::CurveHook for Host {
    fn ed_on_bls12_381_bandersnatch_bandersnatch_te_msm(
        bases: Vec<u8>,
        scalars: Vec<u8>,
    ) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::ed_on_bls12_381_bandersnatch_bandersnatch_te_msm(bases, scalars)
    }
    fn ed_on_bls12_381_bandersnatch_bandersnatch_sw_msm(
        bases: Vec<u8>,
        scalars: Vec<u8>,
    ) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::ed_on_bls12_381_bandersnatch_bandersnatch_sw_msm(bases, scalars)
    }
    fn ed_on_bls12_381_bandersnatch_bandersnatch_te_mul_projective(
        base: Vec<u8>,
        scalar: Vec<u8>,
    ) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::ed_on_bls12_381_bandersnatch_bandersnatch_te_mul_projective(base, scalar)
    }
    fn ed_on_bls12_381_bandersnatch_bandersnatch_sw_mul_projective(
        base: Vec<u8>,
        scalar: Vec<u8>,
    ) -> Result<Vec<u8>, ()> {
        sp_crypto_ec_utils::elliptic_curves::ed_on_bls12_381_bandersnatch_bandersnatch_sw_mul_projective(base, scalar)
    }
}

type EdwardsProjetive = ark_ed_on_bls12_381_bandersnatch::EdwardsProjective<Host>;
type SWProjective = ark_ed_on_bls12_381_bandersnatch::SWProjective<Host>;
```

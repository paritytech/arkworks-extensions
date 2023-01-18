# ark-substrate
## Overview
Disclaimer: Please understand that this is still work based on a WIP [PR](https://github.com/paritytech/substrate/pull/13031) of Substrate and a pre-release of arkworks-rs/algebra 0.4.0 and not ready to be used in production.

Library to integrate [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) into [Substrate](https://github.com/paritytech/substrate). This is a partial fork of the code from [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) and [arkworks-rs/curves](https://github.com/arkworks-rs/curves). We fork the popular elliptic curves `BLS12_381`, `BLS12_377`, `BW6_761`, `ED_ON_BLS12_381` and `ED_ON_BLS12_377` in a way which allows us to replace the elliptic curve arithmetic which is usally slow in WASM by host funciton calls into binary code.

We also provide forks of the models `BW6` and `BLS12`. The reason for this is that we want to avoid the point preparation in the Substrate runtime. Therefore we re-define the elliptic curve sub-groups `G2` for both models as thin wrappers around the affine points and move the point preparation procedure to the host function site.

## Usage

To implement the elliptic curves in Substrate you need to pass the host function calls from the Substrate [sp-io](https://github.com/paritytech/substrate) crate to the instantiated elliptic curves.

See the [groth16 example](https://github.com/achimcc/substrate-groth16) for further implementation details and an example on how to verify a [groth16](https://eprint.iacr.org/2016/260.pdf) proof in a Substrate pallet. 

### BLS12_377
Curve instantiation:

```rust
use sp_ark_bls12_377::{Bls12_377 as Bls12_377_Host}

impl HostFunctions for Host {
    fn bls12_377_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_multi_miller_loop(a, b)
    }
    fn bls12_377_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_final_exponentiation(f12)
    }
    fn bls12_377_msm_g1(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_msm_g1(bases, bigints)
    }
    fn bls12_377_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_mul_projective_g1(base, scalar)
    }
    fn bls12_377_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_mul_affine_g1(base, scalar)
    }
    fn bls12_377_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_msm_g2(bases, bigints)
    }
    fn bls12_377_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_mul_projective_g2(base, scalar)
    }
    fn bls12_377_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_mul_affine_g2(base, scalar)
    }
}

type Bls12_377 = Bls12_377_Host<Host>;
```

### BLS12_381

Curve instantiation:

```rust
use sp_ark_bls12_381::{Bls12_381 as Bls12_381_Host};

pub struct Host {}

impl HostFunctions for Host {
    fn bls12_381_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_381_multi_miller_loop(a, b)
    }
    fn bls12_381_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_381_final_exponentiation(f12)
    }
    fn bls12_381_msm_g1(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_381_msm_g1(bases, bigints)
    }
    fn bls12_381_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_381_mul_projective_g1(base, scalar)
    }
    fn bls12_381_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_381_mul_affine_g1(base, scalar)
    }
    fn bls12_381_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_381_msm_g2(bases, bigints)
    }
    fn bls12_381_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_381_mul_projective_g2(base, scalar)
    }
    fn bls12_381_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_381_mul_affine_g2(base, scalar)
    }
}

type Bls12_381 = Bls12_381_Host<Host>;
```



### BW6_761

Curve instantiation:

```rust
use sp_ark_bw6_761::{BW6_761 as BW6_761_Host}

pub struct Host;

impl HostFunctions for Host {
    fn bw6_761_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_multi_miller_loop(a, b)
    }
    fn bw6_761_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_final_exponentiation(f12)
    }
    fn bw6_761_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_mul_projective_g2(base, scalar)
    }
    fn bw6_761_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_mul_affine_g2(base, scalar)
    }
    fn bw6_761_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_mul_projective_g1(base, scalar)
    }
    fn bw6_761_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_mul_affine_g1(base, scalar)
    }
    fn bw6_761_msm_g1(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_msm_g1(bases, bigints)
    }
    fn bw6_761_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_msm_g2(bases, bigints)
    }
}

type BW6_761 = BW6_761_Host<Host>;
```

### ED_ON_BLS12_377

Curve instatiation:

```rust
use sp_ark_ed_on_bls12_377::{EdwardsProjective as EdwardsProjective_Host}

pub struct Host {}

impl HostFunctions for Host {
    fn ed_on_bls12_377_mul_affine(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_377_mul_affine(base, scalar)
    }
    fn ed_on_bls12_377_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_377_mul_projective(base, scalar)
    }
    fn ed_on_bls12_377_msm(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_377_msm(bases, scalars)
    }
}

type EdwardsProjective = EdwardsProjective_Host<Host>;
```

### ED_ON_BLS12_381

Curve instantiation:

```rust
us sp_ark_ed_on_bls12_381::{SWProjective as SWProjective_Host, EdwardsProjective as EdwardsProjective_Host}

pub struct Host {}

impl HostFunctions for Host {
    fn ed_on_bls12_381_sw_mul_affine(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_381_sw_mul_affine(base, scalar)
    }
    fn ed_on_bls12_381_te_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_381_te_mul_projective(base, scalar)
    }
    fn ed_on_bls12_381_te_mul_affine(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_381_te_mul_affine(base, scalar)
    }
    fn ed_on_bls12_381_sw_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_381_sw_mul_projective(base, scalar)
    }
    fn ed_on_bls12_381_te_msm(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_381_te_msm(bases, scalars)
    }
    fn ed_on_bls12_381_sw_msm(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_381_sw_msm(bases, scalars)
    }
}

type EdwardsProjetive = EdwardsProjective_Host<Host>;
type SWProjective = SWProjective_host<Host>;
```



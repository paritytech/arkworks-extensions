A PR to test the benchmarking report bot

# ark-substrate
## Overview
Disclaimer: Please understand that this is still work based on a WIP [PR](https://github.com/paritytech/substrate/pull/13031) of Substrate and a pre-release of arkworks-rs/algebra 0.4.0 and not ready to be used in production.

Library to integrate [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) into [Substrate](https://github.com/paritytech/substrate). This is a partial fork of the code from [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) and [arkworks-rs/curves](https://github.com/arkworks-rs/curves). We fork the popular elliptic curves `BLS12_381`, `BLS12_377`, `BW6_761`, `ED_ON_BLS12_381` and `ED_ON_BLS12_377` in a way which allows us to replace the elliptic curve arithmetic which is usally slow in WASM by host funciton calls into binary code.

We also provide forks of the models `BW6` and `BLS12`. The reason for this is that we want to avoid the point preparation in the Substrate runtime. Therefore we re-define the elliptic curve sub-groups `G2` for both models as thin wrappers around the affine points and move the point preparation procedure to the host function site.

## Benchmark results

81_msm_te, 10 arguments    |    7813.27       |    3207.47      |${\color{green}\bf 2.44 \boldsymbol{\times}}$|    35.21        |     12470       |      560.82     |
| ed_on_bls12_381_msm_te, 1000 arguments  |    334199.35     |    242277.02    |${\color{green}\bf 1.38 \boldsymbol{\times}}$|    2391.21      |     533490      |      7890       |
| ed_on_bls12_381_mul_projective_te[^*]   |    9.13          |    10.60        |           -           |    7.69         |     22.37       |      0.83       |  
| ed_on_bls12_381_mul_affine_te[^*]       |    5.59          |    10.07        |           -           |    7.61         |     17.25       |      0.37       |
| ed_on_bls12_377_msm, 10 arguments       |    7768.41       |    3192.99      |${\color{green}\bf 2.43 \boldsymbol{\times}}$|    43.24        |     10060       |      553.69     | 
| ed_on_bls12_377_msm, 1000 arguments     |    357890.37     |    267844.08    |${\color{green}\bf 1.34 \boldsymbol{\times}}$|    2465.60      |     537810      |      7680       |
| ed_on_bls12_377_mul_projective[^*]      |    9.41          |    10.32        |           -           |    7.00         |     22.48       |      0.89       |
| ed_on_bls12_377_mul_affine[^*]          |    8.84          |    442.80       |           -           |    8.47         |     22.34       |      0.86       |

[^1]: implemented in a Substrate pallet with [arkworks](https://github.com/arkworks-rs/) library by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^2]: implemented in a Substrate pallet with [ark-substrate](https://github.com/paritytech/ark-substrate) library, executed through host-function call, computed by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^3]: speedup by using ark-substrate and host calls, compared to native speed
[^4]: These extrinsics just receive the arguemnts, deserialize them without using them and then take a generator or zero element of the expected return group, serizlize it and return it. **Calling a host call through a extrinsic which does nothing has been benchmarked with 3.98µs**. Implementation in: https://github.com/achimcc/substrate-arkworks-examples/tree/dummy-calls
[^5]: executed through wasmtime by this repo: [https://github.com/achimcc/native-bench-arkworks](https://github.com/achimcc/wasm-bench-arkworks)
[^6]: native execution, computed by this repo: https://github.com/achimcc/native-bench-arkworks
[^*]: we removed these host calls in the final ark-substrate implementation, since they didn't yield a performance improvement. Implementations can be found in the branches: https://github.com/paritytech/ark-substrate/tree/arkworks-host-function-mul and https://github.com/paritytech/substrate/tree/arkworks-host-function-mul

## Usage

To implement the elliptic curves in Substrate you need to pass the host function calls from the Substrate [sp-io](https://github.com/paritytech/substrate) crate to the instantiated elliptic curves.

See the [substrate-arkworks-examples](https://github.com/achimcc/substrate-arkworks-examples) repo for further implementation details, benchmarks and an example on how to verify a [groth16](https://eprint.iacr.org/2016/260.pdf) proof in a Substrate pallet. 

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



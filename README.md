A PR to test the benchmarking report bot

# ark-substrate
## Overview
Disclaimer: Please understand that this is still work based on a WIP [PR](https://github.com/paritytech/substrate/pull/13031) of Substrate and a pre-release of arkworks-rs/algebra 0.4.0 and not ready to be used in production.

Library to integrate [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) into [Substrate](https://github.com/paritytech/substrate). This is a partial fork of the code from [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) and [arkworks-rs/curves](https://github.com/arkworks-rs/curves). We fork the popular elliptic curves `BLS12_381`, `BLS12_377`, `BW6_761`, `ED_ON_BLS12_381` and `ED_ON_BLS12_377` in a way which allows us to replace the elliptic curve arithmetic which is usally slow in WASM by host funciton calls into binary code.

We also provide forks of the models `BW6` and `BLS12`. The reason for this is that we want to avoid the point preparation in the Substrate runtime. Therefore we re-define the elliptic curve sub-groups `G2` for both models as thin wrappers around the affine points and move the point preparation procedure to the host function site.

## Benchmark results

| extrinsic                                   |  normal(µs)[^1]  |optimized(µs)[^2]|   wasm(µs)[^3]  |  native(µs)[^4] |
| ------------------------------------------- |  --------------- | --------------- | --------------- | --------------- |
| groth16_verification (bls12_381)            |    28790.226     |    8583.25      |     56980       |      4320       | 
| bls12_381_pairing                           |    8727.14       |    1786.77      |     19040       |      1470       |
| bls12_381_msm_g1, 10 arguments              |    386.85        |    109.91       |     737.74      |      73.56      |
| bls12_381_msm_g1, 1000 arguments            |    7658.52       |    3412.56      |     14880       |      1310       |
| bls12_381_msm_g2, 10 arguments              |    691.47        |    137.23       |     1090        |      119.14     |
| bls12_381_msm_g2, 1000 arguments            |    20943.74      |    5935.93      |     36540       |      2630       |
| bls12_381_mul_projective_g1                 |    10.79         |    27.47        |     29.70       |      0.53       |
| bls12_381_mul_affine_g1                     |    8.53          |    21.13        |     39.70       |      0.45       |
| bls12_381_mul_projective_g2                 |    15.68         |    29.69        |     37.74       |      1.43       |
| bls12_381_mul_affine_g2                     |    16.61         |    28.93        |     37.31       |      1.43       |
| bls12_377_pairing                           |    8897.47       |    1848.60      |     18660       |      1560       |
| bls12_377_msm_g1, 10 arguments              |    386.04        |    112.90       |     576.27      |      73.74      | 
| bls12_377_msm_g1, 1000 arguments            |    7606.42       |    3813.22      |     14520       |      1610       |
| bls12_377_msm_g2, 10 arguments              |    739.09        |    159.48       |     1350        |      170.07     |
| bls12_377_msm_g2, 1000 arguments            |    21648.62      |    6461.81      |     37880       |      3860       |
| bls12_377_mul_projective_g1                 |    7.23          |    17.54        |     31.59       |      0.52       |
| bls12_377_mul_affine_g1                     |    6.70          |    16.19        |     61.45       |      0.52       |
| bls12_377_mul_projective_g2                 |    12.99         |    22.23        |     38.19       |      1.69       |
| bls12_377_mul_affine_g2                     |    14.16         |    23.78        |     37.10       |      1.73       |
| bw6_761_pairing                             |    42476.93      |    6077.93      |     87300       |      6950       |
| bw6_761_msm_g1, 10 arguments                |    1219.33       |    260.25       |     1600        |      155.74     |
| bw6_761_msm_g1, 1000 arguments              |    27734.41      |    9746.98      |     51300       |      2950       | 
| bw6_761_msm_g2, 10 arguments                |    1220.31       |    276.68       |     1450        |      151.83     |
| bw6_761_msm_g2, 1000 arguments              |    28499.99      |    9866.41      |     46250       |      2940       |
| bw6_761_mul_projective_g1                   |    17.47         |    54.27        |     44.30       |      1.50       |
| bw6_761_mul_affine_g1                       |    18.66         |    52.85        |     44.28       |      1.52       |
| bw6_761_mul_projective_g2                   |    18.44         |    73.41        |     44.84       |      1.79       |
| bw6_761_mul_affine_g2                       |    22.07         |    77.63        |     44.84       |      1.51       |
| ed_on_bls12_381_msm_sw, 10 arguments        |    279.81        |    58.79        |     345.06      |      58.88      |
| ed_on_bls12_381_msm_sw, 1000 arguments      |    4684.732      |    2470.43      |     8320        |      1140       |
| ed_on_bls12_381_mul_projective_sw           |    5.59          |    10.75        |     24.89       |      0.30       |
| ed_on_bls12_381_mul_affine_sw               |    6.00          |    13.00        |     36.63       |      0.30       |
| ed_on_bls12_381_msm_te, 10 arguments        |    3516.00       |    465.38       |     6540        |      406.76     |
| ed_on_bls12_381_msm_te, 1000 arguments      |    37165.91      |    5952.35      |     72860       |      3070       |
| ed_on_bls12_381_mul_projective_te           |    8.84          |    9.72         |     27.47       |      0.74       |  
| ed_on_bls12_381_mul_affine_te               |    5.25          |    9.64         |     30.05       |      0.29       |
| ed_on_bls12_377_msm, 10 arguments           |    3504.85       |    446.56       |     6070        |      405.37     | 
| ed_on_bls12_377_msm, 1000 arguments         |    37079.82      |    6150.74      |     65890       |      2850       |
| ed_on_bls12_377_mul_projective              |    8.88          |    11.29        |     27.30       |      0.72       |
| ed_on_bls12_377_mul_affine                  |    3585.92       |    437.80       |     6040        |      280.58     |

[^1]: implemented in a Substrate pallet with [arkworks](https://github.com/arkworks-rs/) library by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^2]: implemented in a Substrate pallet with [ark-substrate](https://github.com/paritytech/ark-substrate) library, executed through host-function call, computed by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^3]: executed through wasmtime by this repo: https://github.com/achimcc/native-bench-arkworks
[^4]: native execution, computed by this repo: https://github.com/achimcc/native-bench-arkworks

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



A PR to test the benchmarking report bot

# ark-substrate
## Overview
Disclaimer: Please understand that this is still work based on a WIP [PR](https://github.com/paritytech/substrate/pull/13031) of Substrate and a pre-release of arkworks-rs/algebra 0.4.0 and not ready to be used in production.

Library to integrate [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) into [Substrate](https://github.com/paritytech/substrate). This is a partial fork of the code from [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) and [arkworks-rs/curves](https://github.com/arkworks-rs/curves). We fork the popular elliptic curves `BLS12_381`, `BLS12_377`, `BW6_761`, `ED_ON_BLS12_381` and `ED_ON_BLS12_377` in a way which allows us to replace the elliptic curve arithmetic which is usally slow in WASM by host funciton calls into binary code.

We also provide forks of the models `BW6` and `BLS12`. The reason for this is that we want to avoid the point preparation in the Substrate runtime. Therefore we re-define the elliptic curve sub-groups `G2` for both models as thin wrappers around the affine points and move the point preparation procedure to the host function site.

## Benchmark results

| extrinsic                               |  normal(µs)[^1]  |optimized(µs)[^2]|   speedup[^3]   |  dummy(µs)[^4]  |   wasm(µs)[^5]  |  native(µs)[^6] |
| --------------------------------------- |  --------------- | --------------- | --------------- | --------------- | --------------- | --------------- |
| groth16_verification (bls12_381)        |    23551.78      |    3548.19      |${\color{green}\bf 6.64 \boldsymbol{\times}}$|    5800.99      |                     |      4080       | 
| bls12_381_pairing                       |    10402.36      |    1590.62      |${\color{green}\bf 6.54 \boldsymbol{\times}}$|    448.97       |                     |      1340       |
| bls12_381_msm_g1, 10 arguments          |    7970.50       |    1122.22      |${\color{green}\bf 7.10 \boldsymbol{\times}}$|    87.63        |                     |      578.19     |
| bls12_381_msm_g1, 1000 arguments        |    229069.53     |    35833.72     |${\color{green}\bf 6.39 \boldsymbol{\times}}$|    6486.63      |                     |      11010      |
| bls12_381_msm_g2, 10 arguments          |    24854.55      |    3284.34      |${\color{green}\bf 7.57 \boldsymbol{\times}}$|    10738.18     |                     |      1630       |
| bls12_381_msm_g2, 1000 arguments        |    716298.98     |    101603.89    |${\color{green}\bf 7.05 \boldsymbol{\times}}$|    9896.67      |                     |      32530      |
| bls12_381_mul_projective_g1             |    505.58        |    104.31       |${\color{green}\bf 4.85 \boldsymbol{\times}}$|    12.13        |                     |      48.27      |
| bls12_381_mul_affine_g1                 |    439.51        |    89.42        |${\color{green}\bf 4.92 \boldsymbol{\times}}$|    9.74         |                     |      41.25      |
| bls12_381_mul_projective_g2             |    1498.49       |    231.95       |${\color{green}\bf 6.46 \boldsymbol{\times}}$|    18.22        |                     |      152.62     |
| bls12_381_mul_affine_g2                 |    1255.50       |    201.16       |${\color{green}\bf 6.24 \boldsymbol{\times}}$|    16.41        |                     |      130.11     |
| bls12_377_pairing                       |    8998.99       |    1594.38      |${\color{green}\bf 5.64 \boldsymbol{\times}}$|    16.64        |                     |      1560       |
| bls12_377_msm_g1, 10 arguments          |    6710.72       |    950.38       |${\color{green}\bf 7.06 \boldsymbol{\times}}$|    51.48        |                     |      516.45     | 
| bls12_377_msm_g1, 1000 arguments        |    196176.16     |    30106.65     |${\color{green}\bf 6.52 \boldsymbol{\times}}$|    4484.67      |                     |      11200      |
| bls12_377_msm_g2, 10 arguments          |    22969.00      |    3503.74      |${\color{green}\bf 6.56 \boldsymbol{\times}}$|    89.93        |                     |      1970       |
| bls12_377_msm_g2, 1000 arguments        |    698696.46     |    118429.47    |${\color{green}\bf 5.90 \boldsymbol{\times}}$|    7948.46      |                     |      38570      |
| bls12_377_mul_projective_g1             |    504.24        |    89.33        |${\color{green}\bf 5.64 \boldsymbol{\times}}$|    11.42        |                     |      49.48      |
| bls12_377_mul_affine_g1                 |    419.75        |    80.46        |${\color{green}\bf 5.22 \boldsymbol{\times}}$|    11.11        |                     |      43.05      |
| bls12_377_mul_projective_g2             |    1539.78       |    270.16       |${\color{green}\bf 5.70 \boldsymbol{\times}}$|    16.64        |                     |      186.37     |
| bls12_377_mul_affine_g2                 |    1290.96       |    234.93       |${\color{green}\bf 5.50 \boldsymbol{\times}}$|    17.18        |                     |      161.32     |
| bw6_761_pairing                         |    52506.13      |    6905.97      |${\color{green}\bf 7.60 \boldsymbol{\times}}$|    844.10       |                     |      5940       |
| bw6_761_msm_g1, 10 arguments            |    47190.40      |    5653.72      |${\color{green}\bf 8.35 \boldsymbol{\times}}$|    161.28       |                     |      2790       |
| bw6_761_msm_g1, 1000 arguments          |    1342834.87    |    168826.52    |${\color{green}\bf 7.95 \boldsymbol{\times}}$|    13526.84     |                     |      57820      | 
| bw6_761_msm_g2, 10 arguments            |    47136.15      |    5686.05      |${\color{green}\bf 8.29 \boldsymbol{\times}}$|    161.92       |                     |      2790       |
| bw6_761_msm_g2, 1000 arguments          |    1344407.42    |    168580.08    |${\color{green}\bf 7.97 \boldsymbol{\times}}$|    13633.30     |                     |      57820      |
| bw6_761_mul_projective_g1               |    1927.85       |    305.39       |${\color{green}\bf 6.31 \boldsymbol{\times}}$|    21.99        |                     |      192.79     |
| bw6_761_mul_affine_g1                   |    1598.12       |    265.21       |${\color{green}\bf 6.03 \boldsymbol{\times}}$|    21.35        |                     |      160.41     |
| bw6_761_mul_projective_g2               |    1919.98       |    308.60       |${\color{green}\bf 6.22 \boldsymbol{\times}}$|    21.64        |                     |      187.83     |
| bw6_761_mul_affine_g2                   |    1599.12       |    270.36       |${\color{green}\bf 5.91 \boldsymbol{\times}}$|    21.57        |                     |      159.88     |
| ed_on_bls12_381_msm_sw, 10 arguments    |    4139.53       |    678.09       |${\color{green}\bf 6.10 \boldsymbol{\times}}$|    36.30        |                     |      461.70     |
| ed_on_bls12_381_msm_sw, 1000 arguments  |    108774.04     |    20241.58     |${\color{green}\bf 5.37 \boldsymbol{\times}}$|    2465.60      |                     |      7480       |
| ed_on_bls12_381_mul_projective_sw       |    269.16        |    53.42        |${\color{green}\bf 5.04 \boldsymbol{\times}}$|    6.69         |                     |      28.77      |
| ed_on_bls12_381_mul_affine_sw           |    234.34        |    49.17        |${\color{green}\bf 4.77 \boldsymbol{\times}}$|    6.17         |                     |      25.47      |
| ed_on_bls12_381_msm_te, 10 arguments    |    6124.97       |    891.09       |${\color{green}\bf 6.87 \boldsymbol{\times}}$|    35.21        |                     |      529.61     |
| ed_on_bls12_381_msm_te, 1000 arguments  |    122059.27     |    20473.18     |${\color{green}\bf 5.96 \boldsymbol{\times}}$|    2391.21      |                     |      7450       |
| ed_on_bls12_381_mul_projective_te       |    217.60        |    45.47        |${\color{green}\bf 4.79 \boldsymbol{\times}}$|    7.69         |                     |      24.62      |  
| ed_on_bls12_381_mul_affine_te           |    224.69        |    47.91        |${\color{green}\bf 4.69 \boldsymbol{\times}}$|    7.61         |                     |      25.71      |
| ed_on_bls12_377_msm, 10 arguments       |    6101.68       |    857.74       |${\color{green}\bf 7.11 \boldsymbol{\times}}$|    43.24        |                     |      421.93     | 
| ed_on_bls12_377_msm, 1000 arguments     |    124114.05     |    20309.37     |${\color{green}\bf 6.11 \boldsymbol{\times}}$|    2465.60      |                     |      5790       |
| ed_on_bls12_377_mul_projective          |    216.51        |    45.31        |${\color{green}\bf 4.78 \boldsymbol{\times}}$|    7.00         |                     |      20.37      |
| ed_on_bls12_377_mul_affine              |    213.23        |    43.56        |${\color{green}\bf 4.90 \boldsymbol{\times}}$|    8.47         |                     |      20.92      |

[^1]: implemented in a Substrate pallet with [arkworks](https://github.com/arkworks-rs/) library by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^2]: implemented in a Substrate pallet with [ark-substrate](https://github.com/paritytech/ark-substrate) library, executed through host-function call, computed by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^3]: speedup by using ark-substrate and host calls, compared to native speed
[^4]: These extrinsics just receive the arguemnts, deserialize them without using them and then take a generator or zero element of the expected return group, serizlize it and return it. **Calling a host call through a extrinsic which does nothing has been benchmarked with 3.98µs**. Implementation in: https://github.com/achimcc/substrate-arkworks-examples/tree/dummy-calls
[^5]: executed through wasmtime by this repo: [https://github.com/achimcc/native-bench-arkworks](https://github.com/achimcc/wasm-bench-arkworks)
[^6]: native execution, computed by this repo: https://github.com/achimcc/native-bench-arkworks

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
    fn bls12_377_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_msm_g2(bases, bigints)
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
    fn bls12_381_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_381_msm_g2(bases, bigints)
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



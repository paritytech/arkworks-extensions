A PR to test the benchmarking report bot

# ark-substrate
## Overview
Disclaimer: Please understand that this is still work based on a WIP [PR](https://github.com/paritytech/substrate/pull/13031) of Substrate and a pre-release of arkworks-rs/algebra 0.4.0 and not ready to be used in production.

Library to integrate [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) into [Substrate](https://github.com/paritytech/substrate). This is a partial fork of the code from [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) and [arkworks-rs/curves](https://github.com/arkworks-rs/curves). We fork the popular elliptic curves `BLS12_381`, `BLS12_377`, `BW6_761`, `ED_ON_BLS12_381` and `ED_ON_BLS12_377` in a way which allows us to replace the elliptic curve arithmetic which is usally slow in WASM by host funciton calls into binary code.

We also provide forks of the models `BW6` and `BLS12`. The reason for this is that we want to avoid the point preparation in the Substrate runtime. Therefore we re-define the elliptic curve sub-groups `G2` for both models as thin wrappers around the affine points and move the point preparation procedure to the host function site.

## Benchmark results

| extrinsic                               |  normal(µs)[^1]  |optimized(µs)[^2]|   speedup[^3]   |  dummy(µs)[^4]  |   wasm(µs)[^5]  |  native(µs)[^6] |
| --------------------------------------- |  --------------- | --------------- | --------------- | --------------- | --------------- | --------------- |
| groth16_verification (bls12_381)        |    26535.30      |    8244.31      |${\color{green}\bf 3.22 \boldsymbol{\times}}$|    5800.99      |     45070       |      4040       | 
| bls12_381_pairing                       |    8257.70       |    1448.53      |${\color{green}\bf 5.70 \boldsymbol{\times}}$|    448.97       |     14140       |      1350       |
| bls12_381_msm_g1, 10 arguments          |    16932.20      |    6869.28      |${\color{green}\bf 2.46 \boldsymbol{\times}}$|    87.63        |     24650       |      600.44     |
| bls12_381_msm_g1, 1000 arguments        |    1313899.30    |    653168.11    |${\color{green}\bf 2.01 \boldsymbol{\times}}$|    6486.63      |     191000      |      11160      |
| bls12_381_msm_g2, 10 arguments          |    115465.19     |    23583.63     |${\color{green}\bf 4.90 \boldsymbol{\times}}$|    10738.18     |     185240      |      1660       |
| bls12_381_msm_g2, 1000 arguments        |    10668568.36   |    2458212.20   |${\color{green}\bf 4.34 \boldsymbol{\times}}$|    9896.67      |     14850000    |      33420      |
| bls12_381_mul_projective_g1[^*]         |    8.00          |    21.96        |           -           |    12.13        |     19.85       |      0.45       |
| bls12_381_mul_affine_g1[^*]             |    8.56          |    21.74        |           -           |    9.74         |     39.70       |      0.45       |
| bls12_381_mul_projective_g2[^*]         |    16.88         |    27.87        |           -           |    18.22        |     37.74       |      1.18       |
| bls12_381_mul_affine_g2[^*]             |    15.87         |    27.71        |           -           |    16.41        |     34.40       |      1.19       |
| bls12_377_pairing                       |    10963.00      |    1889.50      |${\color{green}\bf 10.57 \boldsymbol{\times}}$|    16.64        |     15160       |      1520       |
| bls12_377_msm_g1, 10 arguments          |    20745.06      |    9270.83      |${\color{green}\bf 2.24 \boldsymbol{\times}}$|    51.48        |     28620       |      559.16     | 
| bls12_377_msm_g1, 1000 arguments        |    1287941.57    |    831275.64    |${\color{green}\bf 1.55 \boldsymbol{\times}}$|    4484.67      |     1920000     |      11160      |
| bls12_377_msm_g2, 10 arguments          |    131852.78     |    34796.36     |${\color{green}\bf 3.79 \boldsymbol{\times}}$|    89.93        |     162870      |      2020       |
| bls12_377_msm_g2, 1000 arguments        |    10196159.70   |    2781007.89   |${\color{green}\bf 3.67 \boldsymbol{\times}}$|    7948.46      |     14570000    |      40410      |
| bls12_377_mul_projective_g1[^*]         |    6.87          |    17.36        |           -           |    11.42        |     19.38       |      0.44       |
| bls12_377_mul_affine_g1[^*]             |    6.76          |    16.57        |           -           |    11.11        |     24.49       |      0.45       |
| bls12_377_mul_projective_g2[^*]         |    13.80         |    22.24        |           -           |    16.64        |     28.26       |      1.42       |
| bls12_377_mul_affine_g2[^*]             |    13.60         |    22.49        |           -           |    17.18        |     38.94       |      1.46       |
| bw6_761_pairing                         |    44374.64      |    6002.54      |${\color{green}\bf 7.39 \boldsymbol{\times}}$|    844.10       |     55440       |      6940       |
| bw6_761_msm_g1, 10 arguments            |    155393.79     |    53231.17     |${\color{green}\bf 2.92 \boldsymbol{\times}}$|    161.28       |     206610      |      3490       |
| bw6_761_msm_g1, 1000 arguments          |    13384952.55   |    5070669.53   |${\color{green}\bf 2.64 \boldsymbol{\times}}$|    13526.84     |     18010000    |      75270      | 
| bw6_761_msm_g2, 10 arguments            |    141484.94     |    39324.56     |${\color{green}\bf 3.60 \boldsymbol{\times}}$|    161.92       |     212280      |      3430       |
| bw6_761_msm_g2, 1000 arguments          |    12528071.10   |    4732393.47   |${\color{green}\bf 2.65 \boldsymbol{\times}}$|    13633.30     |     18020000    |      75330      |
| bw6_761_mul_projective_g1[^*]           |    17.05         |    53.83        |           -           |    21.99        |     34.82       |      1.79       |
| bw6_761_mul_affine_g1[^*]               |    18.47         |    55.10        |           -           |    21.35        |     35.64       |      1.77       |
| bw6_761_mul_projective_g2[^*]           |    17.45         |    53.65        |           -           |    21.64        |     35.42       |      1.78       |
| bw6_761_mul_affine_g2[^*]               |    17.55         |    54.28        |           -           |    21.57        |     34.68       |      1.78       |
| ed_on_bls12_381_msm_sw, 10 arguments    |    6663.28       |    3686.07      |${\color{green}\bf 1.81 \boldsymbol{\times}}$|    36.30        |     8610        |      376.61     |
| ed_on_bls12_381_msm_sw, 1000 arguments  |    296140.25     |    215932.66    |${\color{green}\bf 1.37 \boldsymbol{\times}}$|    2465.60      |     430700      |      6010       |
| ed_on_bls12_381_mul_projective_sw[^*]   |    5.57          |    10.08        |           -           |    6.69         |     24.89       |      0.36       |
| ed_on_bls12_381_mul_affine_sw[^*]       |    5.51          |    10.12        |           -           |    6.17         |     36.63       |      0.36       |
| ed_on_bls12_381_msm_te, 10 arguments    |    7813.27       |    3207.47      |${\color{green}\bf 2.44 \boldsymbol{\times}}$|    35.21        |     12470       |      560.82     |
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
    fn bls12_377_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_multi_miller_loop(a, b)
    }
    fn bls12_377_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_final_exponentiation(f12)
    }
    fn bls12_377_msm_g1(bases: Vec<u8>, bigints: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_msm_g1(bases, bigints)
    }
    fn bls12_377_msm_g2(bases: Vec<u8>, bigints: Vec<u8>) -> Vec<u8> {
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
    fn bls12_381_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_381_multi_miller_loop(a, b)
    }
    fn bls12_381_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_381_final_exponentiation(f12)
    }
    fn bls12_381_msm_g1(bases: Vec<u8>, bigints: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_381_msm_g1(bases, bigints)
    }
    fn bls12_381_msm_g2(bases: Vec<u8>, bigints: Vec<u8>) -> Vec<u8> {
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
    fn bw6_761_multi_miller_loop(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_multi_miller_loop(a, b)
    }
    fn bw6_761_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_final_exponentiation(f12)
    }
    fn bw6_761_msm_g1(bases: Vec<u8>, bigints: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bw6_761_msm_g1(bases, bigints)
    }
    fn bw6_761_msm_g2(bases: Vec<u8>, bigints: Vec<u8>) -> Vec<u8> {
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
    fn ed_on_bls12_377_msm(bases: Vec<u8>, scalars: Vec<u8>) -> Vec<u8> {
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
    fn ed_on_bls12_381_te_msm(bases: Vec<u8>, scalars: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_381_te_msm(bases, scalars)
    }
    fn ed_on_bls12_381_sw_msm(bases: Vec<u8>, scalars: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::ed_on_bls12_381_sw_msm(bases, scalars)
    }
}

type EdwardsProjetive = EdwardsProjective_Host<Host>;
type SWProjective = SWProjective_host<Host>;
```



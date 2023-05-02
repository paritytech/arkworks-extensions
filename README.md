# ark-substrate
## Overview
Disclaimer: Please understand that this is still work based on a WIP [PR](https://github.com/paritytech/substrate/pull/13031) of Substrate and a pre-release of arkworks-rs/algebra 0.4.2 and arkworks-rs/curves 0.4.0, it is not ready to be used in production.

Library to integrate [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) into [Substrate](https://github.com/paritytech/substrate). This is a partial fork of the code from [arkworks-rs/algebra](https://github.com/arkworks-rs/algebra) and [arkworks-rs/curves](https://github.com/arkworks-rs/curves). We fork the popular elliptic curves `BLS12_381`, `BLS12_377`, `BW6_761`, `ED_ON_BLS12_381_BANDERSNATCH` and `ED_ON_BLS12_377` in a way which allows us to replace the elliptic curve arithmetic which is usally slow in WASM by host funciton calls into binary code.

We also provide forks of the models `BW6` and `BLS12`. The reason for this is that we want to avoid the point preparation in the Substrate runtime. Therefore we re-define the elliptic curve sub-groups `G2` for both models as thin wrappers around the affine points and move the point preparation procedure to the host function site.

## Benchmark results

| extrinsic                               |  arkworkrs(µs)[^1]  |ark-substrate(µs)[^2]|   speedup[^3]   |  dummy(µs)[^4]  |  native(µs)[^5] |
| --------------------------------------- |  --------------- | --------------- | --------------- | --------------- | --------------- |
| groth16_verification (bls12_381)                     |    23335.84      |    3569.35      |${\color{green}\bf 6.54 \boldsymbol{\times}}$|    190.80       |      4080      | 
| bls12_381_pairing                                    |    9092.61       |    1390.80      |${\color{green}\bf 6.54 \boldsymbol{\times}}$|    24.64        |      1340       |
| bls12_381_msm_g1, 10 arguments                       |    6921.99       |    949.58       |${\color{green}\bf 7.29 \boldsymbol{\times}}$|    50.07        |      578.19     |
| bls12_381_msm_g1, 1000 arguments                     |    194969.80     |    30158.23     |${\color{green}\bf 6.46 \boldsymbol{\times}}$|    2169.47      |      11010      |
| bls12_381_msm_g2, 10 arguments                       |    21513.87      |    2870.33      |${\color{green}\bf 7.57 \boldsymbol{\times}}$|    50.06        |      1630       |
| bls12_381_msm_g2, 1000 arguments                     |    621769.22     |    100801.74    |${\color{green}\bf 7.50 \boldsymbol{\times}}$|    3640.63      |      32530      |
| bls12_381_mul_projective_g1                          |    486.34        |    75.01        |${\color{green}\bf 6.48 \boldsymbol{\times}}$|    11.94        |      48.27      |
| bls12_381_mul_affine_g1                              |    420.01        |    79.26        |${\color{green}\bf 5.30 \boldsymbol{\times}}$|    11.11        |      41.25      |
| bls12_381_mul_projective_g2                          |    1498.84       |    210.50       |${\color{green}\bf 7.12 \boldsymbol{\times}}$|    14.63        |      152.62     |
| bls12_381_mul_affine_g2                              |    1234.92       |    214.00       |${\color{green}\bf 5.77 \boldsymbol{\times}}$|    13.17        |      130.11     |
| bls12_377_pairing                                    |    8904.20       |    1449.52      |${\color{green}\bf 6.14 \boldsymbol{\times}}$|    25.88        |      1560       |
| bls12_377_msm_g1, 10 arguments                       |    6592.47       |    902.50       |${\color{green}\bf 7.30 \boldsymbol{\times}}$|    29.20        |      516.45     | 
| bls12_377_msm_g1, 1000 arguments                     |    191793.87     |    28828.95     |${\color{green}\bf 6.65 \boldsymbol{\times}}$|    1307.62      |      11200      |
| bls12_377_msm_g2, 10 arguments                       |    22509.51      |    3251.84      |${\color{green}\bf 6.92 \boldsymbol{\times}}$|    35.06        |      1970       |
| bls12_377_msm_g2, 1000 arguments                     |    632339.00     |    94521.78     |${\color{green}\bf 6.69 \boldsymbol{\times}}$|    2556.48      |      38570      |
| bls12_377_mul_projective_g1                          |    424.21        |    65.68        |${\color{green}\bf 6.46 \boldsymbol{\times}}$|    11.76        |      49.48      |
| bls12_377_mul_affine_g1                              |    363.85        |    65.68        |${\color{green}\bf 5.54 \boldsymbol{\times}}$|    10.50        |      43.05      |
| bls12_377_mul_projective_g2                          |    1339.39       |    212.20       |${\color{green}\bf 6.31 \boldsymbol{\times}}$|    14.56        |      186.37     |
| bls12_377_mul_affine_g2                              |    1122.08       |    208.74       |${\color{green}\bf 5.38 \boldsymbol{\times}}$|    13.08        |      161.32     |
| bw6_761_pairing                                      |    52065.18      |    6791.27      |${\color{green}\bf 7.67 \boldsymbol{\times}}$|    34.70        |      5940       |
| bw6_761_msm_g1, 10 arguments                         |    47050.21      |    5559.53      |${\color{green}\bf 8.46 \boldsymbol{\times}}$|    67.79        |      2790       |
| bw6_761_msm_g1, 1000 arguments                       |    1167536.06    |    143517.21    |${\color{green}\bf 8.14 \boldsymbol{\times}}$|    4630.95      |      57820      | 
| bw6_761_msm_g2, 10 arguments                         |    41055.89      |    4874.46      |${\color{green}\bf 8.42 \boldsymbol{\times}}$|    58.37        |      2790       |
| bw6_761_msm_g2, 1000 arguments                       |    1209593.25    |    143437.77    |${\color{green}\bf 8.43 \boldsymbol{\times}}$|    4345.36      |      57820      |
| bw6_761_mul_projective_g1                            |    1678.86       |    223.57       |${\color{green}\bf 7.51 \boldsymbol{\times}}$|    27.54        |      192.79     |
| bw6_761_mul_affine_g1                                |    1387.87       |    222.05       |${\color{green}\bf 6.25 \boldsymbol{\times}}$|    27.55        |      160.41     |
| bw6_761_mul_projective_g2                            |    1919.98       |    308.60       |${\color{green}\bf 6.22 \boldsymbol{\times}}$|    26.99        |      187.83     |
| bw6_761_mul_affine_g2                                |    1388.21       |    222.47       |${\color{green}\bf 6.24 \boldsymbol{\times}}$|    21.90        |      159.88     |
| ed_on_bls12_381_bandersnatch_msm_sw, 10 arguments    |    3616.81       |    557.96       |${\color{green}\bf 6.48 \boldsymbol{\times}}$|    21.43        |      461.70     |
| ed_on_bls12_381_bandersnatch_msm_sw, 1000 arguments  |    94473.54      |    16254.32     |${\color{green}\bf 5.81 \boldsymbol{\times}}$|    982.29      |      7480       |
| ed_on_bls12_381_bandersnatch_mul_projective_sw       |    235.38        |    40.70        |${\color{green}\bf 5.78 \boldsymbol{\times}}$|    9.03        |      28.77      |
| ed_on_bls12_381_bandersnatch_mul_affine_sw           |    204.04        |    41.66        |${\color{green}\bf 4.90 \boldsymbol{\times}}$|    8.78        |      25.47      |
| ed_on_bls12_381_bandersnatch_msm_te, 10 arguments    |    5427.77       |    744.74       |${\color{green}\bf 7.29 \boldsymbol{\times}}$|    24.05        |      529.61     |
| ed_on_bls12_381_bandersnatch_msm_te, 1000 arguments  |    106610.20     |    16690.71     |${\color{green}\bf 6.39 \boldsymbol{\times}}$|    1195.35      |      7450       |
| ed_on_bls12_381_bandersnatch_mul_projective_te       |    183.29        |    34.63        |${\color{green}\bf 5.29 \boldsymbol{\times}}$|    9.55        |      24.62      |  
| ed_on_bls12_381_bandersnatch_mul_affine_te           |    181.84        |    33.99        |${\color{green}\bf 5.35 \boldsymbol{\times}}$|    9.50        |      25.71      |
| ed_on_bls12_377_msm, 10 arguments                    |    5304.03       |    700.51       |${\color{green}\bf 7.57 \boldsymbol{\times}}$|    24.02        |      421.93     | 
| ed_on_bls12_377_msm, 1000 arguments                  |    105563.53     |    15757.62     |${\color{green}\bf 6.70 \boldsymbol{\times}}$|    1200.45      |      5790       |
| ed_on_bls12_377_mul_projective                       |    179.54        |    32.72        |${\color{green}\bf 5.49 \boldsymbol{\times}}$|    9.72        |      20.37      |
| ed_on_bls12_377_mul_affine                           |    177.53        |    33.24        |${\color{green}\bf 5.34 \boldsymbol{\times}}$|    9.76        |      20.92      |

[^1]: implemented in a Substrate pallet with [arkworks](https://github.com/arkworks-rs/) library by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^2]: implemented in a Substrate pallet with [ark-substrate](https://github.com/paritytech/ark-substrate) library, executed through host-function call, computed by this repo: https://github.com/achimcc/substrate-arkworks-examples
[^3]: speedup by using ark-substrate and host calls, compared to native speed
[^4]: These extrinsics just receive the arguemnts, deserialize them without using them and then take a generator or zero element of the expected return group, serizlize it and return it. **Calling a host call through a extrinsic which does nothing has been benchmarked with 3.98µs**. Implementation in: https://github.com/achimcc/substrate-arkworks-examples/tree/dummy-calls
[^5]: native execution, computed by this repo: https://github.com/achimcc/native-bench-arkworks

## Usage

See the [substrate-arkworks-examples](https://github.com/achimcc/substrate-arkworks-examples) repo for further implementation details, benchmarks and an example on how to verify a [groth16](https://eprint.iacr.org/2016/260.pdf) proof in a Substrate pallet. 


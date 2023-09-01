#!/bin/bash

set -e

if [ -z "$CARGO_REGISTRY_TOKEN" ]
then
      echo "\$CARGO_REGISTRY_TOKEN is empty, please add it to your env variables"
      exit 1
fi

cargo publish -p sp-ark-models
cargo publish -p sp-ark-bls12-377
cargo publish -p sp-ark-ed-on-bls12-377
cargo publish -p sp-ark-bls12-381
cargo publish -p sp-ark-bw6-761
cargo publish -p sp-ark-ed-on-bls12-381-bandersnatch

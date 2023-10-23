#!/bin/bash

set -e

if [ -z "$CARGO_REGISTRY_TOKEN" ]
then
      echo "\$CARGO_REGISTRY_TOKEN is empty, please add it to your env variables"
      exit 1
fi

cargo publish -p ark-models-ext
cargo publish -p ark-bls12-377-ext
cargo publish -p ark-ed-on-bls12-377-ext
cargo publish -p ark-bls12-381-ext
cargo publish -p ark-bw6-761-ext
cargo publish -p ark-ed-on-bls12-381-bandersnatch-ext

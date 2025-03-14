#!/bin/bash

set -e

if [ -z "$CARGO_REGISTRY_TOKEN" ]
then
      echo "\$CARGO_REGISTRY_TOKEN is empty, please add it to your env variables"
      exit 1
fi

echo "Publishing ark-models-ext crate"
cargo publish -p ark-models-ext

echo "Publishing ark-bls12-377-ext crate"
cargo publish -p ark-bls12-377-ext

echo "Publishing ark-ed-on-bls12-377-ext crate"
cargo publish -p ark-ed-on-bls12-377-ext

echo "Publishing ark-bls12-381-ext crate"
cargo publish -p ark-bls12-381-ext

echo "Publishing ark-bw6-761-ext crate"
cargo publish -p ark-bw6-761-ext

echo "Publishing ark-ed-on-bls12-381-bandersnatch-ext crate"
cargo publish -p ark-ed-on-bls12-381-bandersnatch-ext

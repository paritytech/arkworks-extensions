#!/bin/bash

set -e

echo "Publishing ark-models-ext crate"
cargo publish -p ark-models-ext --dry-run

echo "Publishing ark-bls12-377-ext crate"
cargo publish -p ark-bls12-377-ext --dry-run

echo "Publishing ark-ed-on-bls12-377-ext crate"
cargo publish -p ark-ed-on-bls12-377-ext --dry-run

echo "Publishing ark-bls12-381-ext crate"
cargo publish -p ark-bls12-381-ext --dry-run

echo "Publishing ark-bw6-761-ext crate"
cargo publish -p ark-bw6-761-ext --dry-run

echo "Publishing ark-ed-on-bls12-381-bandersnatch-ext crate"
cargo publish -p ark-ed-on-bls12-381-bandersnatch-ext --dry-run

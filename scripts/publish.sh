#!/usr/bin/env bash

set -e

args="$@"
for arg in $args; do
  if [[ "$arg" == "--dry-run" ]]; then
    DRY_RUN=1
    break
  fi
done

if [[ -z $DRY_RUN && -z "$CARGO_REGISTRY_TOKEN" ]]; then
  echo "\$CARGO_REGISTRY_TOKEN is empty, please add it to your env variables"
  exit 1
fi

function publish() {
  local crate=$1
  echo "Publishing $crate crate (args: $args)"
  cargo publish -p $crate $args
}

publish ark-models-ext
publish ark-bls12-377-ext
publish ark-ed-on-bls12-377-ext
publish ark-bls12-381-ext
publish ark-bw6-761-ext
publish ark-ed-on-bls12-381-bandersnatch-ext
publish ark-vesta-ext
publish ark-pallas-ext
publish ark-secp256k1-ext

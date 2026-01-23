#!/usr/bin/env bash

set -e

# All crates in dependency order
ALL_CRATES=(
  ark-models-ext
  ark-bls12-377-ext
  ark-bls12-381-ext
  ark-bn254-ext
  ark-bw6-761-ext
  ark-bw6-767-ext
  ark-ed-on-bls12-377-ext
  ark-ed-on-bls12-381-bandersnatch-ext
  ark-ed-on-bn254-ext
  ark-pallas-ext
  ark-vesta-ext
  ark-secp256k1-ext
)

function show_help() {
  echo "Usage: $0 [OPTIONS]"
  echo ""
  echo "Publish crates to crates.io"
  echo ""
  echo "Options:"
  echo "  -c, --crates <CRATES>  Comma-separated list of crates to publish"
  echo "  -d, --dry-run          Perform a dry run without publishing"
  echo "  -h, --help             Show this help message"
  echo ""
  echo "Available crates:"
  for crate in "${ALL_CRATES[@]}"; do
    echo "  - $crate"
  done
  echo ""
  echo "Examples:"
  echo "  $0                                  # Publish all crates"
  echo "  $0 --dry-run                        # Dry run for all crates"
  echo "  $0 -c ark-bn254-ext                 # Publish only ark-bn254-ext"
  echo "  $0 -c ark-bn254-ext,ark-models-ext  # Publish specific crates"
}

function publish() {
  local crate=$1
  echo "Publishing $crate (cargo args: $CARGO_ARGS)"
  cargo publish -p "$crate" $CARGO_ARGS
}

# Parse arguments
DRY_RUN=""
CRATES=""
CARGO_ARGS=""

while [[ $# -gt 0 ]]; do
  case $1 in
    -h|--help)
      show_help
      exit 0
      ;;
    -d|--dry-run)
      DRY_RUN=1
      CARGO_ARGS="$CARGO_ARGS --dry-run"
      shift
      ;;
    -c|--crates)
      CRATES="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      echo "Use --help for usage information"
      exit 1
      ;;
  esac
done

# Check for registry token if not dry run
if [[ -z $DRY_RUN && -z "$CARGO_REGISTRY_TOKEN" ]]; then
  echo "Error: \$CARGO_REGISTRY_TOKEN is empty"
  echo "Please add it to your environment variables or use --dry-run"
  exit 1
fi

# Determine which crates to publish
if [[ -z "$CRATES" ]]; then
  # Publish all crates
  SELECTED_CRATES=("${ALL_CRATES[@]}")
else
  # Parse comma-separated list
  IFS=',' read -ra SELECTED_CRATES <<< "$CRATES"
fi

# Validate selected crates
for crate in "${SELECTED_CRATES[@]}"; do
  crate=$(echo "$crate" | xargs)  # trim whitespace
  valid=0
  for all_crate in "${ALL_CRATES[@]}"; do
    if [[ "$crate" == "$all_crate" ]]; then
      valid=1
      break
    fi
  done
  if [[ $valid -eq 0 ]]; then
    echo "Error: Unknown crate '$crate'"
    echo "Use --help to see available crates"
    exit 1
  fi
done

# Publish selected crates
echo "Publishing ${#SELECTED_CRATES[@]} crate(s)..."
echo ""

for crate in "${SELECTED_CRATES[@]}"; do
  crate=$(echo "$crate" | xargs)  # trim whitespace
  publish "$crate"
done

echo ""
echo "Done!"

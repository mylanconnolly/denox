#!/bin/bash
set -e

# This script applies patches to dependencies

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
PATCHES_DIR="$PROJECT_ROOT/patches"

# Function to download and patch a crate
patch_crate() {
  local crate_name=$1
  local crate_version=$2
  
  echo "Patching $crate_name-$crate_version..."
  
  # Create the target directory
  mkdir -p "$PATCHES_DIR/$crate_name-$crate_version"
  
  # Download the crate if it doesn't exist
  if [ ! -f "$PATCHES_DIR/$crate_name-$crate_version.crate" ]; then
    echo "Downloading $crate_name-$crate_version..."
    curl -L "https://crates.io/api/v1/crates/$crate_name/$crate_version/download" \
      -o "$PATCHES_DIR/$crate_name-$crate_version.crate"
  fi
  
  # Extract the crate
  cd "$PATCHES_DIR"
  tar -xzf "$crate_name-$crate_version.crate" -C "$crate_name-$crate_version" --strip-components=1
  
  # Apply the patch if it exists
  if [ -f "$PATCHES_DIR/$crate_name-$crate_version.patch" ]; then
    echo "Applying patch to $crate_name-$crate_version..."
    cd "$crate_name-$crate_version"
    patch -p1 < "../$crate_name-$crate_version.patch"
  fi
}

# Patch deno_core
patch_crate "deno_core" "0.341.0"

echo "All patches applied successfully!"

#!/usr/bin/env bash

set -e

# Get the version from the Cargo.toml file
version=$(grep '^version = ' Cargo.toml | head -n1 | cut -d '"' -f2)
echo "$version"
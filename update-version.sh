#!/usr/bin/env bash

if [ -n "$DEBUG" ]; then
    set -x;
fi

set -e;

# Make the workspace version in Cargo.toml matches the last git tag
last=$(git describe --tags --abbrev=0)
last="${last:1}"
if [ -n "$1" ]; then
    last=$1
fi

if [ -z "$last" ]; then
    echo "No tags found"
    exit 1
fi
current=$(grep -oP '^version = "\K[^"]+' Cargo.toml)
if [ -z "$current" ]; then
    echo "No workspace version found"
    exit 1
fi
if [ "$last" == "$current" ]; then
    echo "Workspace version up-to-date"
    exit 0
fi

echo "Updating version from $current to $last"

echo "Updating Cargo.toml"
sed -i "s|^version = \"$current\"|version = \"$last\"|g" Cargo.toml

crates=(prost-validate prost-reflect-validate prost-validate-build prost-validate-derive prost-validate-types prost-validate-derive-core)

for c in "${crates[@]}"; do
  if grep -q "$c = { version = \"$current\"" Cargo.toml; then
    echo "Updating $c in Cargo.toml"
    sed -i "s|$c = { version = \"$current\"|$c = { version = \"$last\"|g" Cargo.toml
  fi
done

# cargo update

# Update README.md files to reference the latest version
for f in $(find . -name "README.md"); do
  if grep -q "$current" $f; then
    echo "Updating $f"
    sed -i "s|$current|$last|g" $f
  fi
done

cargo publish -n --workspace --allow-dirty

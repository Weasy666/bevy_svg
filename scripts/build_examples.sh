#!/usr/bin/env bash
set -euo pipefail

echo "Building examples…"

awk '
  /^\[\[example\]\]/ {
    name=""
    path=""
  }

  /^[[:space:]]*name[[:space:]]*=/ {
    match($0, /"([^"]+)"/, m)
    name = m[1]
  }

  /^[[:space:]]*path[[:space:]]*=/ {
    match($0, /"([^"]+)"/, m)
    path = m[1]
    print name
  }
' Cargo.toml \
| while read -r example; do
    echo "cargo build --example $example"
    cargo build --example "$example"
  done

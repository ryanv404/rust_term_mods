#!/usr/bin/bash

[ ! command -v "cargo" &>/dev/null ] && echo "ERROR: cargo command not found." >&2 && exit 1

[ ! -d "examples" ] && echo "ERROR: no examples directory found." >&2 && exit 1

EXAMPLES=( examples/*.rs )
[ "${#EXAMPLES[@]}" -eq 0 ] && echo "ERROR: no examples found." >&2 && exit 1

for example in "${EXAMPLES[@]}"; do
    file=$( basename "$example" )
    prog="${file%.rs}"
    cargo run --quiet --example "$prog"
done

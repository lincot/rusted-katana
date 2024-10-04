#!/bin/sh

unsafe_count=$(rg -IcU "unsafe[\s]*\{" -g '!check-katas/' -g lib.rs | paste -sd+ | bc)
sed -i "s/\(there are \)[0-9]\+\( \`unsafe\`\)/\1$unsafe_count\2/" README.md

bytes_count=$(rg -Ic "\.bytes\(\)|\.as_bytes\(\)|\.as_bytes_mut\(\)|\.as_mut_vec\(\)" \
  -g '!check-katas/' -g lib.rs | paste -sd+ | bc)
sed -i "s/\(are used \)[0-9]\+\( times\)/\1$bytes_count\2/" README.md

echo check
cargo check --all-features --all-targets --quiet --release

echo clippy
cargo clippy --all-features --all-targets --no-deps --quiet --release

echo update
cargo update --quiet

echo outdated
cargo outdated 2>/dev/null

echo udeps
cargo udeps --quiet 2>/dev/null

echo fmt
cargo fmt

echo run progress-bars
cargo run --package rusted-katana-progress-bars --quiet --release

echo run check-katas
cargo run --package check-katas --quiet --release

echo miri
if [ ! -v NEXTEST_TEST_THREADS ]; then
  n_logical_cores=$(getconf _NPROCESSORS_ONLN)
  export NEXTEST_TEST_THREADS=$((n_logical_cores > 2 ? n_logical_cores - 2 : 1))
fi
cargo miri nextest run --cargo-quiet --all-targets --no-fail-fast \
  --status-level fail --workspace --exclude digital

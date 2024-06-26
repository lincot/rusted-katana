#!/bin/sh

unsafe_count=$(rg -IcU "unsafe[\s]*\{" -g '!check-katas/' -g lib.rs | paste -sd+ | bc)
sed -i "s/\(there are \)[0-9]\+\( \`unsafe\`\)/\1$unsafe_count\2/" README.md

bytes_count=$(rg -Ic "\.bytes\(\)|\.as_bytes\(\)|\.as_bytes_mut\(\)|\.as_mut_vec\(\)" \
  -g '!check-katas/' -g lib.rs | paste -sd+ | bc)
sed -i "s/\(are used \)[0-9]\+\( times\)/\1$bytes_count\2/" README.md

echo check
cargo check --all-features --all-targets --quiet --release
echo clippy
cargo clippy --all-features --all-targets --no-deps --quiet --release -- \
  -D clippy::all \
  -D clippy::pedantic \
  -D clippy::nursery \
  -A clippy::must-use-candidate \
  -A clippy::default-trait-access \
  -A clippy::missing-panics-doc \
  -A clippy::needless-pass-by-value \
  -A clippy::cast-possible-truncation \
  -A clippy::cast-sign-loss \
  -A clippy::cast-lossless \
  -A clippy::match-on-vec-items \
  -A clippy::too-many-lines \
  -A clippy::maybe-infinite-iter \
  -A clippy::missing-errors-doc \
  -A clippy::cast-precision-loss \
  -A clippy::cast-possible-wrap \
  -A clippy::implicit-hasher \
  -A clippy::similar-names \
  -A clippy::float-cmp \
  -A clippy::return-self-not-must-use \
  -A clippy::new-without-default \
  -A clippy::many-single-char-names \
  -A clippy::unused-io-amount \
  -A clippy::type-complexity \
  -A clippy::uninit-assumed-init \
  -A clippy::zero-sized-map-values \
  -A clippy::uninit-vec \
  -A clippy::cognitive-complexity \
  -A clippy::debug-assert-with-mut-call \
  -A clippy::module-name-repetitions \
  -A clippy::suspicious-operation-groupings
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
cargo miri nextest run --all-targets --no-fail-fast --status-level fail

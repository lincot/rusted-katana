#!/bin/sh

cargo update
cargo outdated --exit-code 1 || exit
cargo fmt
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
    -A clippy::type-complexity
cargo run --package check-katas --release
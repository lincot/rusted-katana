[![Codewars badge](https://www.codewars.com/users/lincot/badges/large)](https://www.codewars.com/users/lincot)

# rusted katana

fastest solutions to [Codewars](https://www.codewars.com/r/HLmVMg) katas

## rules

- written in Rust, but importing code written in other languages is allowed

- nontrivial katas have benchmarks to easily compare solutions

- third-party libraries are allowed, even if they aren't accepted by Codewars

- `#![no_std]` everywhere, because we don't need `std`

- solutions are safe, i.e. they don't produce undefined behavior
(checked with miri); *nevertheless, there are 431 `unsafe` blocks*

- even though most of the tests on Codewars have only ASCII input,
they use Rust's standard UTF-8–encoded strings,
so solutions for string katas are made for Unicode input;
*nevertheless,
`.bytes()`, `.as_bytes()`, `.as_bytes_mut()` and `.as_mut_vec()` are used 168 times*

- function signatures and names from solution setups are preserved;
*but are adjusted with clippy*

- linting is done with `./lint.sh`

- large lookup tables are generally not allowed

## external dependencies

vqsort depends on [highway](https://github.com/google/highway)

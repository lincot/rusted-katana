[![Codewars badge](https://www.codewars.com/users/lincot/badges/large)](https://www.codewars.com/users/lincot)

# rusted katana

fastest solutions to [Codewars](https://www.codewars.com/r/HLmVMg) katas
written in Rust

## rules

- nontrivial katas have benchmarks to easily compare solutions

- usage of third-party libraries is allowed even if not accepted by Codewars

- `#![no_std]` everywhere because we don't need `std`

- solutions are safe, meaning that they will panic or return nonsense
on nonsense input rather than causing undefined behavior (checked with miri);
*nevertheless, there are 492 `unsafe` blocks*

- even though most tests on Codewars contain only ASCII input,
they use Rust's standard UTF-8–encoded strings,
so solutions for string katas are made for Unicode input;
*nevertheless,
`.bytes()`|`.as_bytes()`|`.as_bytes_mut()`|`.as_mut_vec()` are used 170 times*

- proposed function signatures and names are preserved;
*but are adjusted with clippy*

- linting is done using `./lint.sh`

- large lookup tables aren't generally allowed

- importing code from other languages is allowed

## external dependencies

vqsort depends on [highway](https://github.com/google/highway)

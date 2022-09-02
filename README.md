[![Codewars badge](https://www.codewars.com/users/lincot/badges/large)](https://www.codewars.com/users/lincot)

# rusted katana

fast solutions to all [Codewars](https://www.codewars.com/r/HLmVMg) katas
written in Rust

## why?

it's not enough to solve a problem, it needs to be solved well

## rules

- nontrivial katas have benchmarks to easily compare solutions

- usage of third-party libraries is allowed even if not accepted by Codewars

- solutions are safe, meaning that they will panic or return nonsense
on nonsense input rather than causing undefined behavior;
*nevertheless, there are 401 `unsafe` blocks*

- even though most tests on Codewars contain only ASCII input,
they use Rust's standard UTF-8–encoded strings,
so solutions for string katas are made for Unicode input;
*nevertheless,
`.bytes()`|`.as_bytes()`|`.as_bytes_mut()`|`.as_mut_vec()` are used 109 times*

- proposed function signatures and names are preserved;
*but are clippy-adjusted*

- linting is done using `./lint.sh`

- (large) lookup tables aren't generally allowed

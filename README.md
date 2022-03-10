[![Codewars badge](https://www.codewars.com/users/lincot/badges/large)](https://www.codewars.com/users/lincot)

# rusted katana

fast solutions to all [Codewars](https://www.codewars.com/r/HLmVMg) katas
written in Rust

## why?

it's not enough to solve a problem, it needs to be solved well

## general rules

- nontrivial katas have benchmarks to easily compare solutions

- usage of third-party libraries is allowed even if not accepted by Codewars

- solutions are safe, meaning that they will panic or return nonsense
on nonsense input rather than causing undefined behavior;
*nevertheless, there are 102 `unsafe` blocks*

- even though most tests on Codewars contain only ASCII input,
they use Rust's standard UTF-8–encoded strings,
so solutions for string katas are made for Unicode input;
*nevertheless, `.bytes()`|`.as_bytes()`|`.as_mut_vec()` are used 77 times*

- proposed function signatures and names are preserved;
*but are clippy-adjusted*

- (large) lookup tables aren't generally allowed

## done

- [8 kyu](https://www.codewars.com/kata/search/rust?q=&r[]=-8&xids=completed&beta=false&order_by=published_at%20asc)
- [7 kyu](https://www.codewars.com/kata/search/rust?q=&r[]=-7&xids=completed&beta=false&order_by=published_at%20asc)

## todo

- [6 kyu](https://www.codewars.com/kata/search/rust?q=&r[]=-6&xids=completed&beta=false&order_by=published_at%20asc)
- [5 kyu](https://www.codewars.com/kata/search/rust?q=&r[]=-5&xids=completed&beta=false&order_by=published_at%20asc)
- [4 kyu](https://www.codewars.com/kata/search/rust?q=&r[]=-4&xids=completed&beta=false&order_by=published_at%20asc)
- [3 kyu](https://www.codewars.com/kata/search/rust?q=&r[]=-3&xids=completed&beta=false&order_by=published_at%20asc)
- [2 kyu](https://www.codewars.com/kata/search/rust?q=&r[]=-2&xids=completed&beta=false&order_by=published_at%20asc)
- [1 kyu](https://www.codewars.com/kata/search/rust?q=&r[]=-1&xids=completed&beta=false&order_by=published_at%20asc)

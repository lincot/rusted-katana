[![Codewars badge](https://www.codewars.com/users/lincot/badges/large)](https://www.codewars.com/users/lincot)

# rusted katana

fastest solutions to [Codewars](https://www.codewars.com/r/HLmVMg) challenges,
written in Rust

current progress:

![progress bars showing how many katas have been solved](progress-bars.png)

## rules

- nontrivial katas have benchmarks to easily compare solutions

- third-party libraries and libraries written in languages other than Rust
are allowed, even if they are not accepted by Codewars

- solutions are safe (more on it in the [safety section](#safety));
*nevertheless, there are 695 `unsafe` blocks*

- even though most of the tests on Codewars have only ASCII input,
they use Rust's standard UTF-8–encoded strings,
so solutions for string katas are made for Unicode input;
*nevertheless,
`.bytes()`, `.as_bytes()`, `.as_bytes_mut()` and `.as_mut_vec()` are used 295 times*

- function signatures and names from solution setups are preserved;
*but are adjusted with clippy*

- linting is done with `./lint.sh`

- large lookup tables are generally not allowed

## safety

rusted katana solutions should not produce undefined behavior despite the heavy
use of `unsafe` blocks; all of the tests and benchmarks are run with miri as
part of the lint script

### allocating memory

a common pattern is to allocate enough memory for subsequent writes; the
capacity calculation should be checked, or a higher bound should be set for
operands, in case arithmetic operations are performed on arbitrary input;
otherwise, it can lead to heap overflow attacks such as in the example below:

```rs
use core::time::Duration;
use std::{sync::Arc, thread};

fn main() {
    thread::spawn(work).join().unwrap();
}

fn work() {
    let a = Arc::new([0u8; 2]); // or Box or Vec

    thread::spawn(move || {
        println!("a is {a:?}");
        thread::sleep(Duration::from_secs(1));
        println!("a is {a:?}");
    });

    let n = usize::MAX - 16; // it could be user input
    #[allow(arithmetic_overflow)]
    let mut v = Vec::<u16>::with_capacity(n * (n + 16));
    for _ in 0..n {
        for _ in 0..n + 16 {
            unsafe {
                let val = 17733; // could be user payload too
                core::ptr::write(v.as_mut_ptr().add(v.len()), val);
                v.set_len(v.len() + 1);
            }
            thread::sleep(Duration::from_millis((v[0] % 8) as _));
        }
    }
}
```

without overflow checks enabled this code prints

```
a is [0, 0]
a is [69, 69]
Segmentation fault (core dumped)
```

an exception to this that doesn't have to be checked is summation performed on
lengths of existing allocations and small values such as

```rs
let v = Vec::with_capacity(some_str.len() + 35);
```

because allocations are capped at `isize::MAX` bytes, not just `usize::MAX`; so,
it's even allowed to multiply their lengths by two

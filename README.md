This is a quick and dirty Rust package that includes biteopt global optimizer
source code https://github.com/avaneev/biteopt

This is for my own personal experiments. It is not for general use.

This package exposes this function:

```rust
pub fn optimize<T, F>(archetype: &T, params: BiteOptParameters, evaluate: F) -> T
where
    T: Vectorizable + Clone,
    F: Fn(T) -> f64,

// This one has getters and setters .iter() and .set_iter() for each field.
pub struct BiteOptParameters {
    iter: i32,         // Number of iterations per attempt
    depth: i32,        // 1 = plain BiteOpt, >1 CBiteOptDeep (see biteopt.h)
    attc: i32,         // How many optimization attempts to perform
    lower_bound: f64,
    upper_bound: f64,
}
```

Given an evaluating function, this will try to find a vector that minimizes the
value from evaluate(), using BiteOpt.

`lower_bound` and `upper_bound` set limits on the values in the vectorized form
of a value of T.

`archetype` tells the shape of the entity we try to optimize. `optimize` will
use `to_vec` on it to figure out the dimension of the problem but it is
otherwise ignored.

Vectorizable is a trait that comes from rcmaes
(https://github.com/Noeda/rcmaes) package; you have to implement a `to_vec()`
and `from_vec` operations that turn a value of type T into a (Vec<f64>,
T::Context) and back.

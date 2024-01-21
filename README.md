# rayon-scan

[![Current Version](https://img.shields.io/crates/v/rayon-scan.svg)](https://crates.io/crates/rayon-scan)
[![Documentation](https://docs.rs/rayon-scan/badge.svg)](https://docs.rs/rayon-scan)
[![License: MIT/Apache-2.0](https://img.shields.io/crates/l/rayon-scan.svg)](#license)

This crate provides a parallel version of the [Iterator scan](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.scan) method, on [Rayon](https://github.com/rayon-rs/rayon)'s `ParallelIterator`. 

Scan is a higher-order function which is similar to fold, but accumulates the intermediate results at each step. Specifically, the *n*th element of the scan iterator is the result of reducing the first *n* elements of the input with the given operation.

The main difference of parallel scan is that the operator must be associative. In a sequential scan, the operation is applied left-to-right on the input, but in a parallel scan, the order is unspecified.

## Usage

```rust
// Iterate over a sequence of numbers `x0, ..., xN`
// and use scan to compute the partial sums
use rayon::prelude::*;
use rayon_scan::ScanParallelIterator;

let partial_sums = [1, 2, 3, 4, 5]
                    .into_par_iter()       // iterating over i32
                    .scan(|a, b| *a + *b,  // add (&i32, &i32) -> i32
                          0)               // identity
                    .collect::<Vec<i32>>();
assert_eq!(partial_sums, vec![1, 3, 6, 10, 15]);
```

## Performance

For a regular prefix sum or product on ints, the parallel overhead is too much to see any improvement with the parallel version. However, sufficently complex operations such as large matrix multiplications can see large performance benefits. 

In order to maximize performance, it is a good idea to limit the amount of splitting, for example by using `.with_min_len()`. Parallel scan has a sequential section which takes linear time in the number of splits.

See https://github.com/rayon-rs/rayon/pull/1036/ for more details on implementation and performance.

## License

Licensed under [Apache 2.0](LICENSE-APACHE) and [MIT](LICENSE-MIT).

> [!NOTE]
> https://github.com/rayon-rs/rayon/pull/1036/ is open to merge this feature to Rayon, and this crate should become obsolete if it merges.
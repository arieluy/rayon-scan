use self::scan::Scan;
use rayon::prelude::ParallelIterator;

mod scan;

pub trait ScanParallelIterator: ParallelIterator {
    /// Folds the items in the iterator using `scan_op`, and produces a
    /// new iterator with all of the intermediate results.
    ///
    /// Specifically, the nth element of the scan iterator will be the
    /// result of reducing the first n elements of the input with `scan_op`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Iterate over a sequence of numbers `x0, ..., xN`
    /// // and use scan to compute the partial sums
    /// use rayon::prelude::*;
    /// use rayon_scan::ScanParallelIterator;
    ///
    /// let partial_sums = [1, 2, 3, 4, 5]
    ///                     .into_par_iter()       // iterating over i32
    ///                     .scan(|a, b| *a + *b,  // add (&i32, &i32) -> i32
    ///                           0)               // identity
    ///                     .collect::<Vec<i32>>();
    /// assert_eq!(partial_sums, vec![1, 3, 6, 10, 15]);
    /// ```
    ///
    /// **Note:** Unlike a sequential `scan` operation, the order in
    /// which `scan_op` will be applied to produce the result is not fully
    /// specified. So `scan_op` should be [associative] or else the results
    /// will be non-deterministic. Also unlike sequential `scan`, there is
    /// no internal state for this operation, so the operation has a
    /// different signature.
    ///
    /// The argument `identity` should be an "identity" value for
    /// `scan_op`, which may be inserted into the sequence as
    /// needed to create opportunities for parallel execution. So, for
    /// example, if you are doing a summation, then `identity` ought
    /// to represent the zero for your type.
    ///
    /// [associative]: https://en.wikipedia.org/wiki/Associative_property
    fn scan<F>(self, scan_op: F, identity: Self::Item) -> Scan<Self::Item, F>
    where
        F: Fn(&Self::Item, &Self::Item) -> Self::Item + Sync + Send,
        <Self as ParallelIterator>::Item: Send + Sync,
    {
        scan::scan(self, scan_op, identity)
    }
}

impl<T: ParallelIterator> ScanParallelIterator for T {}

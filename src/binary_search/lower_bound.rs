pub trait LowerBound<T: Ord> {
    fn lower_bound(&self, x: &T) -> Result<usize, usize>;
}

impl<T: Ord> LowerBound<T> for [T] {
    ///
    /// Returns the first index `i` in the slice such that `self[i] >= x`
    ///
    /// # Arguments
    ///
    /// * `x`: the value to search for
    ///
    /// returns: Result<usize, usize>
    ///
    ///  Ok(i) if the value is found at index `i`, it is the first occurrence of the value
    ///
    /// Err(i) if the value is not found, `i` is the index where the value should be inserted to maintain the sorted order
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn lower_bound(&self, x: &T) -> Result<usize, usize> {
        let mut low = 0;
        let mut high = self.len();
        while low < high {
            let mid = low + (high - low) / 2;
            if &self[mid] >= x {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        if high < self.len() && self[high] == *x {
            Ok(high)
        } else {
            Err(high)
        }
    }
}

mod tests {
    use super::*;
    use std::collections::{HashSet, HashMap};

    use proptest::proptest;
    use proptest::prelude::*;
    use proptest::sample::SizeRange;
    use proptest::strategy::{Strategy, BoxedStrategy};
    use proptest::collection;
    use proptest::collection::vec;

    #[test]
    fn test_lower_bound() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(a.lower_bound(&0), Err(0));
        assert_eq!(a.lower_bound(&1), Ok(0));
        assert_eq!(a.lower_bound(&2), Ok(1));
        assert_eq!(a.lower_bound(&3), Ok(2));
        assert_eq!(a.lower_bound(&4), Ok(3));
        assert_eq!(a.lower_bound(&5), Ok(4));
        assert_eq!(a.lower_bound(&6), Ok(5));
        assert_eq!(a.lower_bound(&7), Ok(6));
        assert_eq!(a.lower_bound(&8), Ok(7));
        assert_eq!(a.lower_bound(&9), Ok(8));
        assert_eq!(a.lower_bound(&10), Err(9));
    }


    fn lower_bound_prop(a: Vec<i32>, x: i32) {
        let mut a = a;
        a.sort();
        let result = a.lower_bound(&x);
        let first_greater_or_equal_index: usize = (|| {
            for i in 0..a.len() {
                if a[i] >= x {
                    return i;
                }
            }
            a.len()
        })();
        match result {
            Ok(i) => assert_eq!(i, first_greater_or_equal_index),
            Err(i) => assert_eq!(i, first_greater_or_equal_index),
        }
    }
    proptest! {
        #![proptest_config(ProptestConfig {
            fork: true,
            // timeout: 1000,
            .. ProptestConfig::default()
        })]
        #[test]
        fn lower_bound(a in vec(0..=100,1..=100_000).boxed(), x in 0..=100) {
            lower_bound_prop(a, x);
        }

    }
}
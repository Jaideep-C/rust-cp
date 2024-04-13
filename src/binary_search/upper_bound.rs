use crate::binary_search::lower_bound::LowerBound;

pub trait UpperBound<T: Ord> {
    fn upper_bound(&self, x: &T) -> Result<usize, usize>;
}

impl<T: Ord> UpperBound<T> for [T] {
    ///
    /// Returns the first index `i` in the slice such that `self[i] > x`
    ///
    /// # Arguments
    ///
    /// * `x`: the value to search for
    ///
    /// returns: Result<usize, usize>
    /// Ok(i) if the value is found at index `i`, it is the last occurrence of the value
    ///
    /// Err(i) if the value is not found, `i` is the index where the value should be inserted to maintain the sorted order
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    fn upper_bound(&self, x: &T) -> Result<usize, usize> {
        let mut low = 0;
        let mut high = self.len();
        while low < high {
            let mid = low + (high - low) / 2;
            if &self[mid] > x {
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
    fn test_higher_bound(){
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(a.upper_bound(&0), Err(0));
        assert_eq!(a.upper_bound(&1), Err(1));
        assert_eq!(a.upper_bound(&2), Err(2));
        assert_eq!(a.upper_bound(&3), Err(3));
        assert_eq!(a.upper_bound(&4), Err(4));
        assert_eq!(a.upper_bound(&5), Err(5));
        assert_eq!(a.upper_bound(&6), Err(6));
        assert_eq!(a.upper_bound(&7), Err(7));
        assert_eq!(a.upper_bound(&8), Err(8));
        assert_eq!(a.upper_bound(&9), Err(9));
        assert_eq!(a.upper_bound(&10), Err(9));
    }

    fn higher_bound_prop(a: Vec<i32>, x: i32) {
        let mut a = a;
        a.sort();
        let result = a.upper_bound(&x);
        let first_greater_index: usize = (|| {
            for i in 0..a.len() {
                if a[i] > x {
                    return i;
                }
            }
            a.len()
        })();
        match result {
            Ok(i) => assert_eq!(i, first_greater_index,"i:{}, first_greater_index: {}, x:{x}",i,first_greater_index),
            Err(i) => assert_eq!(i, first_greater_index,"i:{}, first_greater_index: {}, x:{x}",i,first_greater_index),
        }
    }
    proptest! {
        #![proptest_config(ProptestConfig {
            fork: true,
            // timeout: 1000,
            .. ProptestConfig::default()
        })]
        #[test]
        fn higher_bound(a in vec(0..=100,1..=20).boxed(), x in 0..=100) {
            higher_bound_prop(a, x);
        }

    }
}
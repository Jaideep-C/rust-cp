use std::io::{BufWriter, StdinLock, Stdout, Write};
mod binary_search {
    mod upper_bound {
        use crate::binary_search::lower_bound::LowerBound;
        pub trait UpperBound<T: Ord> {
            fn upper_bound(&self, x: &T) -> Result<usize, usize>;
        }
        impl<T: Ord> UpperBound<T> for [T] {
            #[doc = ""]
            #[doc = " Returns the first index `i` in the slice such that `self[i] > x`"]
            #[doc = ""]
            #[doc = " # Arguments"]
            #[doc = ""]
            #[doc = " * `x`: the value to search for"]
            #[doc = ""]
            #[doc = " returns: Result<usize, usize>"]
            #[doc = " Ok(i) if the value is found at index `i`, it is the last occurrence of the value"]
            #[doc = ""]
            #[doc = " Err(i) if the value is not found, `i` is the index where the value should be inserted to maintain the sorted order"]
            #[doc = ""]
            #[doc = " # Examples"]
            #[doc = ""]
            #[doc = " ```"]
            #[doc = ""]
            #[doc = " ```"]
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
            use proptest::collection;
            use proptest::collection::vec;
            use proptest::prelude::*;
            use proptest::proptest;
            use proptest::sample::SizeRange;
            use proptest::strategy::{BoxedStrategy, Strategy};
            use std::collections::{HashMap, HashSet};
            #[test]
            fn test_higher_bound() {
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
                    Ok(i) => assert_eq!(
                        i, first_greater_index,
                        "i:{}, first_greater_index: {}, x:{x}",
                        i, first_greater_index
                    ),
                    Err(i) => assert_eq!(
                        i, first_greater_index,
                        "i:{}, first_greater_index: {}, x:{x}",
                        i, first_greater_index
                    ),
                }
            }
            proptest! { #! [proptest_config (ProptestConfig { fork : true , .. ProptestConfig :: default () })] # [test] fn higher_bound (a in vec (0 ..= 100 , 1 ..= 20) . boxed () , x in 0 ..= 100) { higher_bound_prop (a , x) ; } }
        }
    }
    mod lower_bound {
        pub trait LowerBound<T: Ord> {
            fn lower_bound(&self, x: &T) -> Result<usize, usize>;
        }
        impl<T: Ord> LowerBound<T> for [T] {
            #[doc = ""]
            #[doc = " Returns the first index `i` in the slice such that `self[i] >= x`"]
            #[doc = ""]
            #[doc = " # Arguments"]
            #[doc = ""]
            #[doc = " * `x`: the value to search for"]
            #[doc = ""]
            #[doc = " returns: Result<usize, usize>"]
            #[doc = ""]
            #[doc = "  Ok(i) if the value is found at index `i`, it is the first occurrence of the value"]
            #[doc = ""]
            #[doc = " Err(i) if the value is not found, `i` is the index where the value should be inserted to maintain the sorted order"]
            #[doc = " # Examples"]
            #[doc = ""]
            #[doc = " ```"]
            #[doc = ""]
            #[doc = " ```"]
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
            use proptest::collection;
            use proptest::collection::vec;
            use proptest::prelude::*;
            use proptest::proptest;
            use proptest::sample::SizeRange;
            use proptest::strategy::{BoxedStrategy, Strategy};
            use std::collections::{HashMap, HashSet};
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
            proptest! { #! [proptest_config (ProptestConfig { fork : true , .. ProptestConfig :: default () })] # [test] fn lower_bound (a in vec (0 ..= 100 , 1 ..= 100_000) . boxed () , x in 0 ..= 100) { lower_bound_prop (a , x) ; } }
        }
    }
}
mod data_structures {
    pub mod segment_tree {
        pub struct SegmentTree<A, QO, NO>
        where
            QO: Fn(&A, &A) -> A,
            NO: Fn(&A, &A) -> A,
        {
            n: usize,
            tree_arr: Vec<A>,
            query_operation: QO,
            node_operation: NO,
            fallback_value: A,
        }
        impl<A: Default + Clone, QO, NO> SegmentTree<A, QO, NO>
        where
            QO: Fn(&A, &A) -> A,
            NO: Fn(&A, &A) -> A,
        {
            #[doc = " # Arguments"]
            #[doc = ""]
            #[doc = " * `inp_arr`: Array on which the segment tree is built"]
            #[doc = " * `query_operation`: It is the operation that is performed on the left and right subtree values to get the current node value"]
            #[doc = " * `node_operation`: It is the operation that is performed on the current node value and the value to be updated"]
            #[doc = " * `fallback_value`: It is the value that is returned when the query range and the current range are disjoint"]
            #[doc = ""]
            #[doc = " returns: SegmentTree<A, QO, NO>"]
            #[doc = ""]
            #[doc = " # Examples"]
            #[doc = ""]
            #[doc = " ```"]
            #[doc = " let inp_arr: Vec<i64> = vec![0; 4];"]
            #[doc = " let mut segment_tree = SegmentTree::new("]
            #[doc = "    &inp_arr,"]
            #[doc = "   |a: &i64, b: &i64| a.max(b).clone(),"]
            #[doc = "   |a: &i64, b: &i64| a + b,"]
            #[doc = "   i64::MIN,"]
            #[doc = " );"]
            #[doc = " segment_tree.update(&1, (0, 3)); // update the value at index 0 to 3 using the node_operation"]
            #[doc = " let max = segment_tree.query((0, inp_arr.len() - 1)); // query the maximum value in the range (0, inp_arr.len() - 1) using the query_operation"]
            #[doc = " ```"]
            pub fn new(
                inp_arr: &Vec<A>,
                query_operation: QO,
                node_operation: NO,
                fallback_value: A,
            ) -> Self {
                let n = inp_arr.len();
                let tree_arr: Vec<A> = vec![Default::default(); 4 * n];
                let mut a = SegmentTree {
                    n,
                    tree_arr,
                    query_operation,
                    node_operation,
                    fallback_value,
                };
                a.__build(inp_arr, 0, (0, n - 1));
                return a;
            }
            #[doc = " # Arguments"]
            #[doc = ""]
            #[doc = " * `inp_arr`: Input array on which the segment tree is built"]
            #[doc = " * `tree_index`: Index of the current node in the segment tree"]
            #[doc = " * `cur_range`: Range of the current node in the segment tree"]
            #[doc = ""]
            #[doc = " returns: ()"]
            fn __build(&mut self, inp_arr: &Vec<A>, tree_index: usize, cur_range: (usize, usize)) {
                let (cl, cr) = cur_range;
                if cl == cr {
                    self.tree_arr[tree_index] = inp_arr[cl].clone();
                    return;
                }
                let cm = (cl + cr) / 2;
                let left_subtree_range = (cl, cm);
                let right_subtree_range = (cm + 1, cr);
                self.__build(inp_arr, tree_index * 2 + 1, left_subtree_range);
                self.__build(inp_arr, tree_index * 2 + 2, right_subtree_range);
                let left_subtree_value = self.tree_arr[tree_index * 2 + 1].clone();
                let right_subtree_value = self.tree_arr[tree_index * 2 + 2].clone();
                self.tree_arr[tree_index] =
                    (self.query_operation)(&left_subtree_value, &right_subtree_value);
            }
            #[doc = " # Arguments"]
            #[doc = ""]
            #[doc = " * `range`: Range of the query"]
            #[doc = ""]
            #[doc = " returns: A"]
            #[doc = ""]
            #[doc = " # Examples"]
            #[doc = ""]
            #[doc = " ```"]
            #[doc = "  segment_tree.query((0, inp_arr.len() - 1)); // query the maximum value in the range (0, inp_arr.len() - 1) using the query_operation"]
            #[doc = " ```"]
            pub fn query(&self, range: (usize, usize)) -> A {
                self.__query(0, range, (0, self.n - 1))
            }
            #[doc = " # Arguments"]
            #[doc = ""]
            #[doc = " * `tree_index`: Index of the current node in the segment tree"]
            #[doc = " * `range`: Range of the query"]
            #[doc = " * `cur_range`: Range of the current node in the segment tree"]
            #[doc = ""]
            #[doc = " returns: `A`"]
            fn __query(
                &self,
                tree_index: usize,
                range: (usize, usize),
                cur_range: (usize, usize),
            ) -> A {
                let (l, r) = range;
                let (cl, cr) = cur_range;
                if l <= cl && cr <= r {
                    return self.tree_arr[tree_index].clone();
                }
                if cr < l || r < cl {
                    return self.fallback_value.clone();
                }
                let cm = (cl + cr) / 2;
                let left_subtree_value = self.__query(tree_index * 2 + 1, range, (cl, cm));
                let right_subtree_value = self.__query(tree_index * 2 + 2, range, (cm + 1, cr));
                return (self.query_operation)(&left_subtree_value, &right_subtree_value);
            }
            #[doc = ""]
            #[doc = ""]
            #[doc = " # Arguments"]
            #[doc = ""]
            #[doc = " * `value`: New Value to be updated"]
            #[doc = " * `range`: Range on which the update is to be performed"]
            #[doc = ""]
            #[doc = " returns: ()"]
            pub fn update(&mut self, value: &A, range: (usize, usize)) {
                self.__update(value, 0, range, (0, self.n - 1));
            }
            #[doc = " # Arguments"]
            #[doc = ""]
            #[doc = " * `value`: New Value to be updated"]
            #[doc = " * `tree_index`: Index of the current node in the segment tree"]
            #[doc = " * `range`: Range on which the update is to be performed"]
            #[doc = " * `cur_range`: Current range of the node in the segment tree"]
            #[doc = ""]
            #[doc = " returns: ()"]
            fn __update(
                &mut self,
                value: &A,
                tree_index: usize,
                range: (usize, usize),
                cur_range: (usize, usize),
            ) {
                let (l, r) = range;
                let (cl, cr) = cur_range;
                if cr < l || r < cl {
                    return;
                }
                if cl == cr {
                    self.tree_arr[tree_index] =
                        (self.node_operation)(&self.tree_arr[tree_index].clone(), &value);
                    return;
                }
                let cm = (cl + cr) / 2;
                let left_node = (cl, cm);
                let right_node = (cm + 1, cr);
                self.__update(value, tree_index * 2 + 1, range, left_node);
                self.__update(value, tree_index * 2 + 2, range, right_node);
                let left_subtree_value = self.tree_arr[tree_index * 2 + 1].clone();
                let right_subtree_value = self.tree_arr[tree_index * 2 + 2].clone();
                self.tree_arr[tree_index] =
                    (self.query_operation)(&left_subtree_value, &right_subtree_value);
            }
        }
    }
    pub mod trie {
        use std::collections::HashMap;
        #[derive(Default, Debug)]
        struct TrieNode {
            word: String,
            children: HashMap<char, TrieNode>,
        }
        #[derive(Default, Debug)]
        pub struct Trie {
            root: TrieNode,
        }
        impl Trie {
            pub fn new() -> Self {
                Trie {
                    root: TrieNode::default(),
                }
            }
            pub fn insert(&mut self, word: &str) {
                let mut current_node = &mut self.root;
                for c in word.chars() {
                    current_node = current_node.children.entry(c).or_default();
                }
                current_node.word = word.to_string();
            }
            pub fn contains(&self, word: &str) -> bool {
                let mut current_node = &self.root;
                for c in word.chars() {
                    match current_node.children.get(&c) {
                        Some(node) => current_node = node,
                        None => return false,
                    }
                }
                !current_node.word.is_empty()
            }
        }
    }
}
mod number_theory {
    pub mod sieve_of_eratosthenes {
        #[allow(dead_code)]
        pub fn sieve_of_eratosthenes(upper_limit: usize) -> Vec<usize> {
            let mut is_prime = vec![true; upper_limit + 1];
            is_prime[0] = false;
            is_prime[1] = false;
            let mut primes = Vec::new();
            for num in 2..=upper_limit {
                if is_prime[num] {
                    primes.push(num);
                    let mut multiple = num * num;
                    while multiple <= upper_limit {
                        is_prime[multiple] = false;
                        multiple += num;
                    }
                }
            }
            primes
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            #[test]
            fn test_sieve_of_eratosthenes_logic() {
                let primes = sieve_of_eratosthenes(10usize.pow(4));
                for p in primes {
                    let mut num = 1;
                    let mut cnt = 0;
                    while num <= p {
                        if p % num == 0 {
                            cnt += 1;
                        }
                        num += 1;
                    }
                    assert_eq!(cnt, 2);
                }
            }
            #[test]
            fn test_sieve_of_eratosthenes_time() {
                let primes = sieve_of_eratosthenes(10usize.pow(8));
            }
        }
    }
    pub mod modulo_arithmetic {
        pub struct ModuloArithmetic {
            modulo: i64,
        }
        #[allow(dead_code)]
        impl ModuloArithmetic {
            pub fn new(modulo: i64) -> Self {
                Self { modulo }
            }
            pub fn def() -> Self {
                Self {
                    modulo: 1_000_000_007,
                }
            }
            pub fn add(&self, a: i64, b: i64) -> i64 {
                return (a + b) % self.modulo;
            }
            pub fn sub(&self, a: i64, b: i64) -> i64 {
                return (a - b + self.modulo) % self.modulo;
            }
            pub fn mul(&self, a: i64, b: i64) -> i64 {
                return (a * b) % self.modulo;
            }
            pub fn div(&self, a: i64, b: i64) -> i64 {
                return self.mul(a, self.bin_pow(b, self.modulo - 2));
            }
            pub fn bin_pow(&self, a: i64, b: i64) -> i64 {
                if b == 0 {
                    return 1;
                }
                let half = self.bin_pow(a, b / 2);
                return if b % 2 == 0 {
                    self.mul(half, half)
                } else {
                    self.mul(self.mul(half, half), a)
                };
            }
            pub fn fact(&self, n: i64) -> i64 {
                let mut ans = 1;
                for i in 1..=n {
                    ans = self.mul(ans, i);
                }
                return ans;
            }
        }
        #[cfg(test)]
        mod tests {
            use super::*;
            const MODULO: i64 = 1_000_000_007;
            #[test]
            fn test_modulo_arithmetic() {
                let modulo_arithmetic = ModuloArithmetic::new(MODULO);
                assert_eq!(modulo_arithmetic.add(1, 2), 3);
                assert_eq!(modulo_arithmetic.sub(2, 2), 0);
                assert_eq!(modulo_arithmetic.mul(2, 2), 4);
                assert_eq!(modulo_arithmetic.bin_pow(2, 2), 4);
                assert_eq!(modulo_arithmetic.div(4, 2), 2);
            }
            #[test]
            fn test_add() {
                let modulo_arithmetic = ModuloArithmetic::new(MODULO);
                for _ in 0..1_000_000_000 {
                    let a = rand::random::<i64>() % MODULO;
                    let b = rand::random::<i64>() % MODULO;
                    let expected = (a + b) % MODULO;
                    let actual = modulo_arithmetic.add(a, b);
                    assert_eq!(actual, expected);
                }
            }
            #[test]
            fn test_sub() {
                let modulo_arithmetic = ModuloArithmetic::new(MODULO);
                for _ in 0..1_000_000_000 {
                    let a = rand::random::<i64>() % MODULO;
                    let b = rand::random::<i64>() % MODULO;
                    let expected = (a - b + MODULO) % MODULO;
                    let actual = modulo_arithmetic.sub(a, b);
                    assert_eq!(actual, expected);
                }
            }
            #[test]
            fn test_mul() {
                let modulo_arithmetic = ModuloArithmetic::new(MODULO);
                for _ in 0..1_000_000_000 {
                    let a = rand::random::<i64>() % MODULO;
                    let b = rand::random::<i64>() % MODULO;
                    let expected = (a * b) % MODULO;
                    let actual = modulo_arithmetic.mul(a, b);
                    assert_eq!(actual, expected);
                }
            }
            #[test]
            fn test_pow() {
                let modulo_arithmetic = ModuloArithmetic::new(MODULO);
                for _ in 0..1_000_000_000 {
                    let a = rand::random::<i64>() % MODULO;
                    let b = rand::random::<u32>();
                    let expected = (a.pow(b)) % MODULO;
                    let actual = modulo_arithmetic.bin_pow(a, b as i64);
                    assert_eq!(actual, expected);
                }
            }
            #[test]
            fn test_div() {
                let modulo_arithmetic = ModuloArithmetic::new(MODULO);
                for _ in 0..1_000_000_000 {
                    let a = rand::random::<i64>() % MODULO;
                    let b = rand::random::<i64>() % MODULO;
                    let expected = (a / b) % MODULO;
                    let actual = modulo_arithmetic.div(a, b);
                    assert_eq!(actual, expected);
                }
            }
            #[test]
            fn test_div_odd() {
                let modulo_arithmetic = ModuloArithmetic::new(MODULO);
                let mut a = 7;
                let mut b = 3;
                let mut expected = (a / b) % MODULO;
                let mut actual = modulo_arithmetic.div(a - (a % b), b);
                assert_eq!(actual, expected);
            }
        }
    }
}
#[allow(dead_code)]
const MODULO: i64 = 1_000_000_007;
const TESTCASE_AVAILABLE: bool = true;
#[warn(unused_macros)]
macro_rules ! fwriteln { ($ dst : expr ,$ ($ arg : tt) *) => { writeln ! ($ dst ,$ ($ arg) *) ; $ dst . flush () ; } ; }
macro_rules ! debugln { ($ ($ es : expr) ,+) => { # [cfg (debug_assertions)] { eprintln ! ($ ($ es) ,+) } } ; }
macro_rules ! debug { ($ ($ es : expr) ,+) => { # [cfg (debug_assertions)] { eprint ! ($ ($ es) ,+) } } ; }
#[allow(unused_must_use)]
fn solve(scanner: &mut Scanner<StdinLock>, buf: &mut BufWriter<Stdout>) {}
#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use proptest::collection;
    use proptest::prelude::*;
    use proptest::proptest;
    use proptest::sample::SizeRange;
    use proptest::strategy::{BoxedStrategy, Strategy};
    use std::collections::{HashMap, HashSet};
    #[doc = " Generates a `Vec<T>` where `T` is generated by `element_generator` and the size of the vector is within `size_range`."]
    #[doc = ""]
    #[doc = " # Example"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " let element_generator = (0..10).prop_map(|x| x).boxed();"]
    #[doc = " let size_range = 5..=100;"]
    #[doc = " let vector_generator = vec_generator(element_generator, size_range);"]
    #[doc = " ```"]
    fn vec_generator<T: 'static + std::fmt::Debug>(
        element_generator: BoxedStrategy<T>,
        size_range: impl Into<SizeRange>,
    ) -> BoxedStrategy<Vec<T>> {
        collection::vec(element_generator, size_range).boxed()
    }
    #[doc = " Generates a `HashSet<T>` where `T` is generated by `element_generator` and the size of the set is within `size_range`."]
    #[doc = ""]
    #[doc = " # Example"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " let element_generator = (0..10).prop_map(|x| x).boxed();"]
    #[doc = " let size_range = 5..=100;"]
    #[doc = " let set_generator = hash_set_generator(element_generator, size_range);"]
    #[doc = " ```"]
    fn hash_set_generator<T: 'static + std::hash::Hash + std::cmp::Eq + std::fmt::Debug>(
        element_generator: BoxedStrategy<T>,
        size_range: impl Into<SizeRange>,
    ) -> BoxedStrategy<HashSet<T>> {
        collection::hash_set(element_generator, size_range).boxed()
    }
    #[doc = " Generates a `HashMap<K, V>` where `K` is generated by `key_generator`, `V` is generated by `value_generator`, and the size of the map is within `size_range`."]
    #[doc = ""]
    #[doc = " # Example"]
    #[doc = ""]
    #[doc = " ```"]
    #[doc = " let key_generator = (0..10).prop_map(|x| x).boxed();"]
    #[doc = " let value_generator = (0..10).prop_map(|x| x).boxed();"]
    #[doc = " let size_range = 5..=100;"]
    #[doc = " let map_generator = hash_map_generator(key_generator, value_generator, size_range);"]
    #[doc = " ```"]
    fn hash_map_generator<
        K: 'static + std::hash::Hash + std::cmp::Eq + std::fmt::Debug,
        V: 'static + std::fmt::Debug,
    >(
        key_generator: BoxedStrategy<K>,
        value_generator: BoxedStrategy<V>,
        size_range: std::ops::Range<usize>,
    ) -> BoxedStrategy<HashMap<K, V>> {
        collection::hash_map(key_generator, value_generator, size_range).boxed()
    }
    proptest! { #! [proptest_config (ProptestConfig { fork : true , cases : 100_000 , .. ProptestConfig :: default () })] }
}
fn main() {
    let mut scanner = Scanner::new(std::io::stdin().lock());
    let mut buf = BufWriter::new(std::io::stdout());
    let t: i64 = if TESTCASE_AVAILABLE {
        scanner.next()
    } else {
        1
    };
    for _ in 0..t {
        solve(&mut scanner, &mut buf);
    }
}
pub struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    offset: usize,
}
impl<R: std::io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            offset: 0,
        }
    }
    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if self.offset < self.buf_str.len() {
                let pos = self.buf_str[self.offset..]
                    .iter()
                    .position(|&c| c == b' ')
                    .unwrap_or(self.buf_str.len() - self.offset);
                let token = std::str::from_utf8(&self.buf_str[self.offset..self.offset + pos])
                    .expect("non utf8")
                    .trim();
                self.offset += pos + 1;
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.offset = 0;
        }
    }
    pub fn next_collection<T, C>(&mut self, n: usize) -> C
    where
        T: std::str::FromStr,
        C: std::iter::FromIterator<T>,
    {
        (0..n).map(|_| self.next()).collect()
    }
    pub fn next_pair<T1: std::str::FromStr, T2: std::str::FromStr>(&mut self) -> (T1, T2) {
        (self.next(), self.next())
    }
}

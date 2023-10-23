pub struct SegmentTree<A, QO, NO>
    where
        QO: Fn(&A, &A) -> A,
        NO: Fn(&A, &A) -> A {
    n: usize,
    tree_arr: Vec<A>,
    query_operation: QO,
    node_operation: NO,
    fallback_value: A,
}

impl<A: Default + Clone, QO, NO> SegmentTree<A, QO, NO>
    where
        QO: Fn(&A, &A) -> A,
        NO: Fn(&A, &A) -> A {
    /// # Arguments
    ///
    /// * `inp_arr`: Array on which the segment tree is built
    /// * `query_operation`: It is the operation that is performed on the left and right subtree values to get the current node value
    /// * `node_operation`: It is the operation that is performed on the current node value and the value to be updated
    /// * `fallback_value`: It is the value that is returned when the query range and the current range are disjoint
    ///
    /// returns: SegmentTree<A, QO, NO>
    ///
    /// # Examples
    ///
    /// ```
    /// let inp_arr: Vec<i64> = vec![0; 4];
    /// let mut segment_tree = SegmentTree::new(
    ///    &inp_arr,
    ///   |a: &i64, b: &i64| a.max(b).clone(),
    ///   |a: &i64, b: &i64| a + b,
    ///   i64::MIN,
    /// );
    /// segment_tree.update(&1, (0, 3)); // update the value at index 0 to 3 using the node_operation
    /// let max = segment_tree.query((0, inp_arr.len() - 1)); // query the maximum value in the range (0, inp_arr.len() - 1) using the query_operation
    /// ```
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
    /// # Arguments
    ///
    /// * `inp_arr`: Input array on which the segment tree is built
    /// * `tree_index`: Index of the current node in the segment tree
    /// * `cur_range`: Range of the current node in the segment tree
    ///
    /// returns: ()
    fn __build(
        &mut self,
        inp_arr: &Vec<A>,
        tree_index: usize,
        cur_range: (usize, usize),
    ) {
        let (cl, cr) = cur_range;
        // if it is a leaf node
        if cl == cr {
            self.tree_arr[tree_index] = inp_arr[cl].clone();
            return;
        }
        // else process the left and right subtree until we reach the leaf node
        let cm = (cl + cr) / 2;
        let left_subtree_range = (cl, cm);
        let right_subtree_range = (cm + 1, cr);

        // build the left and right subtree
        self.__build(inp_arr, tree_index * 2 + 1, left_subtree_range);
        self.__build(inp_arr, tree_index * 2 + 2, right_subtree_range);

        // merge the left and right subtree values to get the current node value
        let left_subtree_value = self.tree_arr[tree_index * 2 + 1].clone();
        let right_subtree_value = self.tree_arr[tree_index * 2 + 2].clone();
        self.tree_arr[tree_index] = (self.query_operation)(&left_subtree_value, &right_subtree_value);
    }
    /// # Arguments
    ///
    /// * `range`: Range of the query
    ///
    /// returns: A
    ///
    /// # Examples
    ///
    /// ```
    ///  segment_tree.query((0, inp_arr.len() - 1)); // query the maximum value in the range (0, inp_arr.len() - 1) using the query_operation
    /// ```
    pub fn query(&self, range: (usize, usize)) -> A {
        self.__query(0, range, (0, self.n - 1))
    }
    /// # Arguments
    ///
    /// * `tree_index`: Index of the current node in the segment tree
    /// * `range`: Range of the query
    /// * `cur_range`: Range of the current node in the segment tree
    ///
    /// returns: `A`
    fn __query(
        &self,
        tree_index: usize,
        range: (usize, usize),
        cur_range: (usize, usize),
    ) -> A {
        let (l, r) = range;
        let (cl, cr) = cur_range;
        // if cur_range is inside range
        if l <= cl && cr <= r {
            return self.tree_arr[tree_index].clone();
        }
        // if cur_range and range are disjoint, return the fallback value
        if cr < l || r < cl {
            return self.fallback_value.clone();
        }
        // else process the left and right subtree until the subtree range is inside the query range or disjoint
        let cm = (cl + cr) / 2;
        let left_subtree_value = self.__query(
            tree_index * 2 + 1,
            range,
            (cl, cm),
        );
        let right_subtree_value = self.__query(
            tree_index * 2 + 2,
            range,
            (cm + 1, cr),
        );
        return (self.query_operation)(&left_subtree_value, &right_subtree_value);
    }
    ///
    ///
    /// # Arguments
    ///
    /// * `value`: New Value to be updated
    /// * `range`: Range on which the update is to be performed
    ///
    /// returns: ()
    pub fn update(
        &mut self,
        value: &A,
        range: (usize, usize),
    ) {
        self.__update(
            value,
            0,
            range,
            (0, self.n - 1),
        );
    }
    /// # Arguments
    ///
    /// * `value`: New Value to be updated
    /// * `tree_index`: Index of the current node in the segment tree
    /// * `range`: Range on which the update is to be performed
    /// * `cur_range`: Current range of the node in the segment tree
    ///
    /// returns: ()
    fn __update(
        &mut self,
        value: &A,
        tree_index: usize,
        range: (usize, usize),
        cur_range: (usize, usize),
    ) {
        let (l, r) = range;
        let (cl, cr) = cur_range;
        // if cur_range and range are disjoint
        if cr < l || r < cl {
            return;
        }
        // when we reach the leaf level
        if cl == cr {
            self.tree_arr[tree_index] = (self.node_operation)(&self.tree_arr[tree_index].clone(), &value);
            return;
        }
        // else process the left and right subtree until we reach the leaf node
        let cm = (cl + cr) / 2;
        let left_node = (cl, cm);
        let right_node = (cm + 1, cr);
        self.__update(
            value,
            tree_index * 2 + 1,
            range,
            left_node,
        );
        self.__update(
            value,
            tree_index * 2 + 2,
            range,
            right_node,
        );
        let left_subtree_value = self.tree_arr[tree_index * 2 + 1].clone();
        let right_subtree_value = self.tree_arr[tree_index * 2 + 2].clone();
        self.tree_arr[tree_index] = (self.query_operation)(&left_subtree_value, &right_subtree_value);
    }
}



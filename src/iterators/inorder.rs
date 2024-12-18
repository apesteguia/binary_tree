use crate::BinaryTree;
use std::fmt;

pub struct InOrderIter<'a, T>
where
    T: 'a + Clone + Ord + Eq + fmt::Debug,
{
    stack: Vec<&'a BinaryTree<T>>,
}

impl<'a, T> InOrderIter<'a, T>
where
    T: 'a + Clone + Ord + Eq + fmt::Debug,
{
    fn new(tree: &'a BinaryTree<T>) -> Self {
        let mut iter = InOrderIter { stack: Vec::new() };
        iter.push_left(tree);
        iter
    }

    fn push_left(&mut self, node: &'a BinaryTree<T>) {
        let mut current = node;
        while let Some(left) = &current.left {
            self.stack.push(current);
            current = left;
        }
        self.stack.push(current);
    }
}

impl<'a, T> Iterator for InOrderIter<'a, T>
where
    T: 'a + Clone + Ord + Eq + fmt::Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }

        let top = self.stack.pop().unwrap();
        if let Some(ref right) = &top.right {
            self.push_left(right);
        }

        top.elem.as_ref().map(|v| &**v)
    }
}

impl<'a, T> IntoIterator for &'a BinaryTree<T>
where
    T: 'a + Clone + Ord + Eq + fmt::Debug,
{
    type Item = &'a T;
    type IntoIter = InOrderIter<'a, T>;

    /// Creates a iterator that iterates in INORDER
    /// Basic usage:
    /// ```
    /// let mut a = BinaryTree::new(1);
    /// a.vec_insert([1,2,3]);
    /// for i in a.into_iter() {
    ///     println!("{}", *i);
    /// }
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        InOrderIter::new(self)
    }
}

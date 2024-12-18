pub struct BinaryTree<T>
where
    T: Clone + Ord + Eq + std::fmt::Debug,
{
    pub elem: Option<Box<T>>,
    pub right: Option<Box<BinaryTree<T>>>,
    pub left: Option<Box<BinaryTree<T>>>,
}

//pub type BinaryTreeNode<T> = BinaryTree<T>;

impl<T> BinaryTree<T>
where
    T: std::fmt::Debug + Clone + Ord,
{
    pub fn new(elem: T) -> BinaryTree<T> {
        BinaryTree {
            elem: Some(Box::new(elem)),
            right: None,
            left: None,
        }
    }

    pub fn delete(&mut self, del_elem: T) -> Option<T> {
        if let Some(ref mut elem) = self.elem {
            if del_elem < **elem {
                if let Some(left) = &mut self.left {
                    left.delete(del_elem)
                } else {
                    None
                }
            } else if del_elem > **elem {
                if let Some(right) = &mut self.right {
                    right.delete(del_elem)
                } else {
                    None
                }
            } else {
                // Element found, now delete it
                match (self.left.take(), self.right.take()) {
                    (None, right) => {
                        *self = BinaryTree {
                            elem: None,
                            left: None,
                            right,
                        };
                        Some(del_elem)
                    }
                    (left, None) => {
                        *self = BinaryTree {
                            elem: None,
                            left,
                            right: None,
                        };
                        Some(del_elem)
                    }
                    (Some(left), Some(right)) => {
                        let min_right = right.min().cloned();
                        *self = BinaryTree {
                            elem: min_right.map(Box::new),
                            left: Some(left),
                            right: Some(right),
                        };
                        Some(del_elem)
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn insert(&mut self, new_elem: T) {
        match &mut self.elem {
            Some(ref mut elem) => {
                if new_elem < **elem {
                    if let Some(left) = &mut self.left {
                        left.insert(new_elem)
                    } else {
                        self.left = Some(Box::new(BinaryTree::new(new_elem)));
                    }
                } else if new_elem > **elem {
                    if let Some(right) = &mut self.right {
                        right.insert(new_elem)
                    } else {
                        self.right = Some(Box::new(BinaryTree::new(new_elem)));
                    }
                }
            }
            None => {
                self.elem = Some(Box::new(new_elem));
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.elem.is_none() && self.right.is_none() && self.left.is_none()
    }

    pub fn contains(&self, search_elem: T) -> bool {
        match &self.elem {
            Some(ref elem) => {
                if search_elem == **elem {
                    true
                } else if search_elem < **elem {
                    match &self.left {
                        Some(left) => left.contains(search_elem),
                        None => false,
                    }
                } else {
                    match &self.right {
                        Some(right) => right.contains(search_elem),
                        None => false,
                    }
                }
            }
            None => false,
        }
    }

    pub fn clear(&mut self) {
        self.elem = None;
        self.left = None;
        self.right = None;
    }

    pub fn max(&self) -> Option<&T> {
        match &self.right {
            Some(right) => right.max(),
            None => self.elem.as_ref().map(|boxed_elem| &**boxed_elem),
        }
    }

    pub fn min(&self) -> Option<&T> {
        match &self.left {
            Some(left) => left.min(),
            None => self.elem.as_ref().map(|boxed_elem| &**boxed_elem),
        }
    }

    pub fn height(&self) -> usize {
        match (&self.left, &self.right) {
            (Some(left), Some(right)) => 1 + usize::max(left.height(), right.height()),
            (Some(left), None) => 1 + left.height(),
            (None, Some(right)) => 1 + right.height(),
            (None, None) => 1,
        }
    }

    pub fn count(&self) -> usize {
        let left_count = self.left.as_ref().map_or(0, |left| left.count());
        let right_count = self.right.as_ref().map_or(0, |right| right.count());
        let current_count = if self.elem.is_some() { 1 } else { 0 };
        left_count + right_count + current_count
    }

    pub fn inorder(&self) {
        if let Some(left) = &self.left {
            left.inorder();
        }
        if let Some(elem) = &self.elem {
            println!("{:?}", elem);
        }
        if let Some(right) = &self.right {
            right.inorder();
        }
    }

    pub fn preorder(&self) {
        if let Some(elem) = &self.elem {
            println!("{:?}", elem);
        }
        if let Some(left) = &self.left {
            left.preorder();
        }
        if let Some(right) = &self.right {
            right.preorder();
        }
    }

    pub fn postorder(&self) {
        if let Some(left) = &self.left {
            left.postorder();
        }
        if let Some(right) = &self.right {
            right.postorder();
        }
        if let Some(elem) = &self.elem {
            println!("{:?}", elem);
        }
    }

    pub fn vec_insert(&mut self, elems: Vec<T>) {
        for i in elems {
            let _ = self.insert(i);
        }
    }
}

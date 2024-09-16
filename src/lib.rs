pub struct BinaryTree<T>
where
    T: std::fmt::Debug + Clone + Ord,
{
    elem: Option<Box<T>>,
    right: Option<Box<BinaryTree<T>>>,
    left: Option<Box<BinaryTree<T>>>,
}

pub type BinaryTreeNode<T> = BinaryTree<T>;

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

    pub fn delete(&mut self, del_elem: T) -> Result<(), String> {
        if let Some(ref mut elem) = self.elem {
            if del_elem < **elem {
                if let Some(left) = &mut self.left {
                    left.delete(del_elem)
                } else {
                    Err(format!(
                        "Element {:?} not found in the left subtree",
                        del_elem
                    ))
                }
            } else if del_elem > **elem {
                if let Some(right) = &mut self.right {
                    right.delete(del_elem)
                } else {
                    Err(format!(
                        "Element {:?} not found in the right subtree",
                        del_elem
                    ))
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
                        Ok(())
                    }
                    (left, None) => {
                        *self = BinaryTree {
                            elem: None,
                            left,
                            right: None,
                        };
                        Ok(())
                    }
                    (Some(left), Some(right)) => {
                        let min_right = right.min().cloned();
                        *self = BinaryTree {
                            elem: min_right.map(Box::new),
                            left: Some(left),
                            right: Some(right),
                        };
                        Ok(())
                    }
                }
            }
        } else {
            Err(format!("Element {:?} not found in the tree", del_elem))
        }
    }

    pub fn insert(&mut self, new_elem: T) -> Result<(), String> {
        match &mut self.elem {
            Some(ref mut elem) => {
                if new_elem < **elem {
                    if let Some(left) = &mut self.left {
                        left.insert(new_elem)
                    } else {
                        self.left = Some(Box::new(BinaryTree::new(new_elem)));
                        Ok(())
                    }
                } else if new_elem > **elem {
                    if let Some(right) = &mut self.right {
                        right.insert(new_elem)
                    } else {
                        self.right = Some(Box::new(BinaryTree::new(new_elem)));
                        Ok(())
                    }
                } else {
                    Err(format!("Element {:?} already exists in the tree", new_elem))
                }
            }
            None => {
                self.elem = Some(Box::new(new_elem));
                Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min() {
        let mut a = BinaryTree::new(1);
        a.vec_insert([1, 2, 3, 4, 5, 6].to_vec());
        assert_eq!(a.min(), Some(&1));
    }

    #[test]
    fn test_max() {
        let mut a = BinaryTree::new("hola");
        a.vec_insert(["a", "b", "c", "adfasdfafas", "zzz"].to_vec());
        assert_eq!(a.max(), Some(&"zzz"));
    }

    #[test]
    fn test_string() {
        let mut a = BinaryTree::new(String::from("Hola"));
        let b: Vec<String> = vec!["Hola".to_string(), "adios".to_string()];
        a.vec_insert(b);
        assert_eq!(a.min(), Some(&"Hola".to_string()));
    }

    #[test]
    fn test_insert() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5).unwrap();
        tree.insert(15).unwrap();
        assert_eq!(tree.contains(5), true);
        assert_eq!(tree.contains(15), true);
        assert_eq!(tree.contains(10), true);
    }

    #[test]
    fn test_delete() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5).unwrap();
        tree.insert(15).unwrap();
        tree.delete(5).unwrap();
        assert_eq!(tree.contains(5), false);
        assert_eq!(tree.contains(15), true);
    }

    #[test]
    fn test_contains() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5).unwrap();
        tree.insert(15).unwrap();
        assert_eq!(tree.contains(5), true);
        assert_eq!(tree.contains(20), false);
    }

    #[test]
    fn test_clear() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5).unwrap();
        tree.insert(15).unwrap();
        tree.clear();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_is_empty() {
        let mut tree: BinaryTree<i32> = BinaryTree {
            elem: None,
            left: None,
            right: None,
        };
        assert!(tree.is_empty());

        tree.insert(10).unwrap();
        assert!(!tree.is_empty());
    }

    #[test]
    fn test_height() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5).unwrap();
        tree.insert(15).unwrap();
        tree.insert(20).unwrap();
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn test_count() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5).unwrap();
        tree.insert(15).unwrap();
        tree.insert(20).unwrap();
        assert_eq!(tree.count(), 4); // Contando el nodo raíz
    }
}

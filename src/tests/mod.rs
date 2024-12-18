#[cfg(test)]
mod tests {
    use crate::bstree::bstree::BinaryTree;

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
        tree.insert(5);
        tree.insert(15);
        assert_eq!(tree.contains(5), true);
        assert_eq!(tree.contains(15), true);
        assert_eq!(tree.contains(10), true);
    }

    #[test]
    fn test_delete() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5);
        tree.insert(15);
        tree.delete(5).unwrap();
        assert_eq!(tree.contains(5), false);
        assert_eq!(tree.contains(15), true);
    }

    #[test]
    fn test_contains() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5);
        tree.insert(15);
        assert_eq!(tree.contains(5), true);
        assert_eq!(tree.contains(20), false);
    }

    #[test]
    fn test_clear() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5);
        tree.insert(15);
        tree.clear();
        assert!(tree.is_empty());
    }

    #[test]
    fn test_height() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(20);
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn test_count() {
        let mut tree = BinaryTree::new(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(20);
        assert_eq!(tree.count(), 4); // Contando el nodo raíz
    }
}

## Simple implementation of a binary tree.

### Example
```rust
fn main() {
    let mut a = BinaryTree::new(1);
    a.insert(2);
    a.vec_insert([1,2,3]);
    a.delete(1);
    for i in a.into_iter() {
        println!("{}", *i);
    }
}
```

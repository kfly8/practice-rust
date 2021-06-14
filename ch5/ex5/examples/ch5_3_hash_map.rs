use std::collections::HashMap;

fn main() {
    let mut m1 = HashMap::new();

    m1.insert("a", 1);
    m1.insert("b", 3);
    assert_eq!(m1.len(), 2);
    assert_eq!(m1.get("b"), Some(&3));
    assert_eq!(m1.get("c"), None);
}

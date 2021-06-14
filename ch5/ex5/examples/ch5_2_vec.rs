fn main() {
    let v1 = vec![false, true, false];
    let v2 = vec![0.0, -1.0, 1.2, 0.5];

    assert_eq!(v1.len(), 3);
    assert_eq!(v2.len(), 4);

    let v3 = vec![0; 100];
    assert_eq!(v3.len(), 100);
}

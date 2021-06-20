fn main() {
    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));
    assert_eq!(a1.get(10), None);

    let o1 = Some(10);
    match o1 {
        Some(s) => assert_eq!(s, 10),
        None => unreachable!(),
    }

    let o2 = Some(20);
    if let Some(s) = o2 {
        assert_eq!(s, 20);
    }

    assert_eq!(add_elems(&[3, 7, 31, 127]), Some(3 + 127));
    assert_eq!(add_elems(&[7, 11]), None);
}

fn add_elems(s: &[i32]) -> Option<i32> {
    let s0 = s.get(0)?;
    let s3 = s.get(3)?;
    Some(s0 + s3)
}

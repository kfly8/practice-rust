pub fn main() {
    let c1 = 'A';
    let c2 = 'a';

    assert!(c1 < c2);
    assert!(c1.is_uppercase());
    assert!(c2.is_lowercase());

    let c3 = '0';
    assert!(c3.is_digit(10));

    // 絵文字
    println!("{}", '\u{1f600}');

    assert_eq!(std::mem::size_of::<char>(), 4);
}

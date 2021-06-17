pub fn main() {
    let mut s1 = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");

    let s3 = "ストロベリー".to_string();

    s1.push_str("タルト");
    assert_eq!(s1, "ラズベリータルト");

    s2.push('と');

    // s2.push_str(s3); s3はString型だから、NG
    s2.push_str(&s3); // 型強制で&Stringから&strに変換

    assert_eq!(s2, "ブラックベリーとストロベリー");
}

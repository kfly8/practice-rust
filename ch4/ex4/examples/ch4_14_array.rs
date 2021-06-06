fn main() {
    let a1 = [false, true, false];
    let a2 = [0.0, 1.5, 1.0, -2.0];

    assert_eq!(a1.len(), 3);
    assert_eq!(a2.len(), 4);

    assert_eq!(a1[1], true);
    let i = 2;
    assert_eq!(a2[i], 1.0);

    let a3 = [0; 100];
    assert_eq!(a3.len(), 100);
    assert_eq!(a3[12], 0);

    {
        let size = 100;

        // 配列の長さは実行時に指定できない
        // error[E0435]: attempt to use a non-constant value in a constant
        // let a1 = [0; size];

        let mut v1 = vec![0; size];
        assert_eq!(v1.len(), 100);

        assert_eq!(v1[0], 0);
        v1.push(123);
        assert_eq!(v1[100], 123);
        assert_eq!(v1.len(), 101);

        // expected enum `Option`, found integer
        // help: try using a variant of the expected enum: `Some(*right_val)`
        assert_eq!(v1.pop(), Some(123));

        assert_eq!(v1.len(), 100);
    }
}

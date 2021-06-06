fn main() {
    let t1 = (88, true);

    assert_eq!(t1.0, 88);
    assert_eq!(t1.1, true);

    //変数は使えない
    //let i = 0;
    //assert_eq!(t1.i, 88);

    {
        let mut t1 = (88, true);
        t1.0 += 100;
        assert_eq!(t1.0, 188);
        println!("{}", t1.0);
    }

    {
        let (num, flg) = (88, true);

        assert_eq!(num, 88);
        assert_eq!(flg, true);
    }

    {
        let t = (88, true);
        let (num, flg) = t;
        assert_eq!(num, 88);
        assert_eq!(flg, true);
    }

    {
        let ((x1, y1), (x2, y2)) = ((0, 5), (10, -1));
        assert_eq!(x1, 0);
        assert_eq!(y1, 5);
        assert_eq!(x2, 10);
        assert_eq!(y2, -1);
    }

    {
        let ((x1, y1), _) = ((0, 5), (10, -1));
        assert_eq!(x1, 0);
        assert_eq!(y1, 5);
    }

    {
        let mut t1 = ((0, 5), (10, -1));

        let ((ref mut x1_ptr, ref mut y1_ptr), _) = t1;
        *x1_ptr += 3;
        *y1_ptr *= -1;

        assert_eq!(t1.0, (3, -5));
        assert_eq!(t1.1, (10, -1));
    }
}

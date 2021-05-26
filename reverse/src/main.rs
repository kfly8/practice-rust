fn main() {
    let mut x = [1, 2, 3, 5, 8, 13];
    println!("x is {:?}", x);

    reverse(&mut x);

    println!("reversed x is {:?}", x)
}

fn reverse(x: &mut [u32]) {
    for i in 0..x.len() / 2 {
        let j = x.len() - 1 - i;
        x.swap(i, j);
    }
}

fn add0(s0: &str, s1: &str) -> Result<i32, String> {
    let s0 = s0.parse::<i32>().map_err(|_e| "s0が整数でない")?;
    let s1 = s1.parse::<i32>().map_err(|_e| "s1が整数でない")?;
    Ok(s0 + s1)
}

fn main() {
    assert_eq!("10".parse::<i32>(), Ok(10));
    let res0 = "a".parse::<i32>();
    assert!(res0.is_err());
    println!("{:?}", res0);

    assert_eq!(add0("3", "127"), Ok(130));
    assert_eq!(add0("3", "abc"), Err("s1が整数でない".to_string()));
}



fn main() {
    println!("Hello, world!");
    let v = vec![1, 2, 4, 5, 6];
    let third: &i32 = &v[2];
    println!("3番目の文字は {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("3番目の要素は {third}"),
        None => println!("3番目の要素に値がありません"),
    }
}

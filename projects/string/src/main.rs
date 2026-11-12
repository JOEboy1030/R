fn main() {
    let data = "こんにちは、妖怪ウォッチ";
    let s = data.to_string();
    println!("{} を所持しています。", s);
    drop(s);

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{} を所持しています。", s);

    let mut s1 = String::from("foo");
    // let s2 = "bar";
    s1.push_str(&s);
    println!("s is {s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);
}

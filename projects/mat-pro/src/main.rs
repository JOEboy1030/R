fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(val) => Some(val * 100),
    }
}

fn main() {
    let a: i32 = 100;
    let b: i32 = 1000;
    let c = None;

    let th = plus_one(Some(a));
    let th2 = plus_one(Some(b));
    let th3 = plus_one(c);

    println!("{:?}, {:?}, {:?}", th, th2, th3);
}

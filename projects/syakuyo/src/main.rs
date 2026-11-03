fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);

    let a = String::from("World hello");
    first_word(&a);
    println!("{a}");

    

}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


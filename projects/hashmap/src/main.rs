use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Red")).or_insert(100);
    scores.entry(String::from("Blue")).or_insert(100);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    // println!("{}", score);

    for (key, value) in &scores {
        println!("{key}:{value}");
    }

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    let mut number = vec![10, 30, 4, 6, 32, 93, 1, 65, 4, 90, 7, 42, 28, 6, 8];
    number.sort();
    let mid_index = number.len() / 2;
    let mut median = 0;

    for (i, &val) in number.iter().enumerate() {
        if i == mid_index {
            median = val;
            break;
        }
    }
    println!("ソートの結果:{:?}", number);
    println!("中央値の結果:{}", median);
}

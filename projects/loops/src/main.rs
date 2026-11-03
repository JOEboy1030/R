fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 3 {
            break counter * 2;
        }

        println!("Hello, world!");
    };

    println!("{result}");

    let a = String::from("Hello-rust");
    let b = a.clone();
    println!("{} -> {}", a, b);

    let s = String::from("hello"); // sがスコープに入る

    takes_ownership(s); // sの値が関数にムーブされ...
                        // ... ここではもう有効ではない

    let x = 5; // xがスコープに入る

    makes_copy(x); // xも関数にムーブされるが、
                   // i32はCopyなので、この後にxを使っても
                   // 大丈夫
    println!("{x}");
} // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、
  // 何も特別なことは起こらない。

fn takes_ownership(some_string: String) {
    // some_stringがスコープに入る。
    println!("{}", some_string);
} // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。
  // 後ろ盾してたメモリが解放される。

fn makes_copy(some_integer: i32) {
    // some_integerがスコープに入る
    println!("{}", some_integer);
} // ここでsome_integerがスコープを抜ける。何も特別なことはない。

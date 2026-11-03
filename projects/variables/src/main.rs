use std::io;

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
    // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    // 入力された値は数字ではありません

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    // {index}番目の要素の値は{element}です

    another_function(5,'j');

}

fn another_function(x:i32,y:char) {
    println!("Another function. {x} : {y}");  // 別の関数
}

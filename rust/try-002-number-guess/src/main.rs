use std::io;

fn main() {
    println!("hello world");
    println!("输入一个数字");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("wrong");

    println!("你猜到的数字：{guess}")
}

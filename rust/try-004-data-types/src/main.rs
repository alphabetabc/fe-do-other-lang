/**
 * Rust 是 静态类型（statically typed）语言
 * 在编译时就必须知道所有变量的类型。
 * ---
 * 标量（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。
 */

fn main() {
    let condition = true;

    let mut number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}

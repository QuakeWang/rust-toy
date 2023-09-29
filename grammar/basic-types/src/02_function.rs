// 函数名和变量名使用蛇形命名法，例如 fn add_two() -> {}
// 函数的位置可以随便放，Rust 不关心在哪里定义了函数，只要有定义即可
// 每个函数参数都需要标注类型
#![allow(unused)]
fn add(i: i32, j: i32) -> i32 {
    i + j
}

// 函数的返回值就是函数体最后一条表达式的返回值，也可以使用 return 提前返回
fn plus_or_minus(x: i32) -> i32 {
    if x > 5 {
        return x - 5;
    }
    x + 5
}

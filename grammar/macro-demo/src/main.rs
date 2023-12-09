macro_rules! add {
    ($($a:expr), *) => {
        {
            // 开头有个 0 处理没有任何参数传入的情况
            0
            // 重复的块
            $(+ $a)*
        }
    }
}

fn main() {
    let x = 0;
    let sum01 = add!(1, 2, 3, 4);
    let sum02 = add!(x);
    println!("{}", sum01);
    println!("{}", sum02);
}

fn variable_change() {
    // Rust 的变量默认情况下是不可变的。 let x = 5;
    // mut 关键字可以让变量变为可变的
    let mut x = 5;
    println!("The value of x is :{}", x);
    x = 6;
    println!("The value of x is :{}", x);
}

fn variable_ignore() {
    // 使用下划线开头忽略未使用的变量、方法
    let _y = 10;
}

fn variable_deconvolution() {
    // let 表达式不仅仅用于变量的绑定，还能进行复杂变量的解构：从一个相对复杂的变量中，匹配出该变量的一部分内容：
    let (a, mut b): (bool, bool) = (true, false);
    // a = true 不可变，b = false 可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;
    assert_eq!(a, b);

    // 解构式赋值
    struct Struct {
        e: i32,
    }

    let (a, b, c, d, e);
    (a, b) = (1, 2);
    [c, .., d, _] = [1, 2, 3, 4, 5];
    (Struct { e, .. } = Struct { e: 5 });
    // 这种方式和上面的 let 保持了一致性，但是 let 会重新绑定，而这里仅仅是对之前绑定的变量进行再赋值
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}

fn shadowing() {
    let x = 5;
    // 在 shadowing 函数的作用域内对之前的 x 进行遮蔽
    let x = x + 1;
    {
        // 在当前的花括号作用域内，对之前的 x 进行遮蔽
        let x = x * 2;
        println!("The value of x in the inner scope is :{}", x);
    }

    println!("The value of x is {}", x);
}

fn main() {
    variable_change();
    variable_ignore();
    variable_deconvolution();
    shadowing();
}

fn main() {
    let mut name1 = String::from("hello");
    let mut name2 = String::from("halo");

    // 捕获 &mut name
    let mut c1 = || {
        name1.push_str("Quake");
        println!("c: {}", name1);
    };

    let mut c2 = move || {
        name2.push_str("!");
        println!("c1: {}", name2);
    };

    c1();
    c2();

    call_mut(&mut c1);
    call_mut(&mut c2);

    call_once(c1);
    call_once(c2);
}

// 在作为参数时，FnMut 也要显式地使用 mut，或者 &mut
fn call_mut(c: &mut impl FnMut()) {
    c();
}

fn call_once(c: impl FnOnce()) {
    c();
}

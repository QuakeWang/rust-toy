fn main() {
    pop();
    remove();
    truncate();
    clear();
}

fn pop() {
    // pop：刪除並返回字符串的最後一個字符
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);
}

fn remove() {
    // remove：刪除並返回字符串指定位置的字符
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    // 删除第一个汉字
    string_remove.remove(0);
    // 下面代码会发生错误
    // string_remove.remove(1);
    // 直接删除第二个汉字
    // string_remove.remove(3);
    dbg!(string_remove);
}

fn truncate() {
    // truncate：刪除字符串中從指定位置開始到結尾的全部字符
    let mut string_truncate = String::from("测试 truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);
}

fn clear() {
    // clear：清空字符串
    let mut string_clear = String::from("string clear");
    string_clear.clear();
    dbg!(string_clear);
}

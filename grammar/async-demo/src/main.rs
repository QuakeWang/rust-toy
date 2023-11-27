fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // 注意这里 block_on，里面是异步代码
            println!("Hello world!");
        })
}

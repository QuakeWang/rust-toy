use tokio::{fs::File, io::AsyncReadExt};

async fn doit() -> std::io::Result<()> {
    let mut file = File::open("foo.txt").await.unwrap();
    let mut contents = vec![];
    // 将文件内容读到 contents 动态数组里面，注意传入的是可变引用
    file.read_to_end(&mut contents).await.unwrap();
    println!("len = {}", contents.len());
    Ok(())
}

#[tokio::main]
async fn main() {
    let _ = doit().await;
}

use tokio::{fs::File, io::AsyncWriteExt};

async fn doit() -> std::io::Result<()> {
    let mut file = File::create("foo.txt").await.unwrap();
    file.write_all(b"hello world!").await.unwrap();
    Ok(())
}

#[tokio::main]
async fn main() {
    let _ = doit().await;
}

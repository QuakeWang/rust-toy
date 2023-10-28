use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9527").await?;
    loop {
        let (stream, addr) = listener.accept().await?;
        println!("acceptd: {:?}", addr);
        // The default of LengthDelimitedCodec is 4 bit
        let mut stream = Framed::new(stream, LengthDelimitedCodec::new());

        tokio::spawn(async move {
            // Only can accept the message itself
            while let Some(Ok(data)) = stream.next().await {
                println!("Got: {:?}", String::from_utf8_lossy(&data));
                // Framed/LengthDelimitedCodec will automatically compute and add length
                stream.send(Bytes::from("goodbye world!")).await.unwrap();
            }
        });
    }
}

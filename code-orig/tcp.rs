use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>()?;
    let listener = TcpListener::bind(&addr).await?;
    println!("Server listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // Read data from the stream and print it
    let mut buffer = vec![0; 1024];
    while let Ok(n) = stream.read(&mut buffer).await {
        if n == 0 {
            break; // End of stream
        }
        let data = &buffer[..n];
        println!("Received: {:?}", data);
    }

    Ok(())
}

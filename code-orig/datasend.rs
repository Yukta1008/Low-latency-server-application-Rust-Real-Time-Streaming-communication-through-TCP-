use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:8080".parse().expect("Invalid address");
    let listener = TcpListener::bind(&addr).await?;
    println!("Server listening on: {}", addr);

    while let Ok((mut stream, _)) = listener.accept().await {
        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];
            while let Ok(n) = stream.read(&mut buffer).await {
                if n == 0 {
                    break;
                }
                if let Err(e) = stream.write_all(&buffer[..n]).await {
                    eprintln!("Error writing to socket: {}", e);
                    break;
                }
            }
        });
    }

    Ok(())
}

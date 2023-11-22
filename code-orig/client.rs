use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... your existing main function ...

    // Simulate a client that sends data to the server
    tokio::spawn(simulate_client());

    // ... rest of your main function ...
}

async fn simulate_client() {
    loop {
        let data = "This is a test message from the client";
        let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
        let mut stream = TcpStream::connect(addr).await.unwrap();

        if let Err(e) = stream.write_all(data.as_bytes()).await {
            eprintln!("Failed to send data to server: {}", e);
        }

        sleep(Duration::from_secs(1)).await;
    }
}

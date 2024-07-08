use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use std::error::Error;

async fn handle_client(mut socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];

    loop {
        match timeout(Duration::from_secs(10), socket.read(&mut buffer)).await {
            Ok(Ok(bytes_read)) => {
                if bytes_read == 0 {
                    return Ok(());
                }

                println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));

                // Echo the received message back to the client
                socket.write_all(&buffer[..bytes_read]).await?;
            }
            Ok(Err(e)) => return Err(Box::new(e)),
            Err(_) => {
                println!("Connection timed out");
                return Ok(());
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server listening on 0.0.0.0:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("failed to handle client: {:?}", e);
            }
        });
    }
}


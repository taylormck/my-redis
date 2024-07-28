use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // Explicitly avoid creating the buffer on the stack
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // 0 means the remote has closed
                    Ok(0) => return,
                    Ok(n) => {
                        // Copy data back to the socket
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // Unexpected error. There's nothing we can do.
                            return;
                        }
                    }
                    Err(_) => {
                        // Unexpected error. There's nothing we can do.
                        return;
                    }
                }
            }
        });
    }
}

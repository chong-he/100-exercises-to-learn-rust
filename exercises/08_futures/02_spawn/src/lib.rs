use tokio::net::TcpListener;

// TODO: write an echo server that accepts TCP connections on two listeners, concurrently.
//  Multiple connections (on the same listeners) should be processed concurrently.
//  The received data should be echoed back to the client.
pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {\
    // by using tokio::spawn, listener 1 is in its own task, listener 2 is in its own task
    // so using tokio::spawn, both listeners can accept connections at the same time / concurrently, they don't block each other
    // the listener here refers to the server (the echo function)
    let listener_one = tokio::spawn(echo(first));
    let listener_two = tokio::spawn(echo(second));

    // both listeners (one and two) run at the same time
    // this is to execute both servers (listeners) at the same time
    // if for example, it is like this:
    // let outcome1 = listener_one.await;
    // let outcome2 = listener_two.await;
    // then this runs sequentially (not concurrently)
    let (outcome1, outcome2) = tokio::join!(listener_one, listener_two);

    // one ? is for the JoinHandle result
    // one ? is for the output of this function, i.e., Result<(), E>
    outcome1??;
    outcome2??;
    Ok(())
}

async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    // This is the same function echo as in chap8sec01
    // this is the server, so it runs forever
    loop {
        let (mut stream, address) = listener.accept().await?;

        tokio::spawn(async move {
            let (mut reader, mut writer) = stream.split();

            tokio::io::copy(&mut reader, &mut writer).await;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        tokio::spawn(echoes(first_listener, second_listener));

        let requests = vec!["hello", "world", "foo", "bar"];
        let mut join_set = JoinSet::new();

        for request in requests.clone() {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, mut writer) = socket.split();

                    // Send the request
                    writer.write_all(request.as_bytes()).await.unwrap();
                    // Close the write side of the socket
                    writer.shutdown().await.unwrap();

                    // Read the response
                    let mut buf = Vec::with_capacity(request.len());
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, request.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}

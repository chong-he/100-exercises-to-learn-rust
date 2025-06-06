use tokio::net::TcpListener;

// TODO: write an echo server that accepts incoming TCP connections and
//  echoes the received data back to the client.
//  `echo` should not return when it finishes processing a connection, but should
//  continue to accept new connections.
//
// Hint: you should rely on `tokio`'s structs and methods to implement the echo server.
// In particular:
// - `tokio::net::TcpListener::accept` to process the next incoming connection
// - `tokio::net::TcpStream::split` to obtain a reader and a writer from the socket
// - `tokio::io::copy` to copy data from the reader to the writer
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    // this echo server does the below:
    // 1. accepts incoming TCP connections
    // 2. read the data that the client sends
    // 3. send the same data back to client

    // loop is because the server always runs, never stops. so the server always accept new connections
    loop {
        // this is to accept incoming connection (from a client)
        // .accept() returns 2 outputs, a tuple, so we let the output to be a (connection, client's address) tuple
        // .accept() blocks until a client connects, it is waiting for a client
        let (mut stream, address) = listener.accept().await?;

        tokio::spawn(async move {
            // .split() returns 2 outputs, read and write
            // TCP connections are bidirectional (can read and write)
            // here, "stream" is the TCP connections (TcpStream)
            // read is for receiving data from the client
            // write is for sending back data to the client
            let (mut reader, mut writer) = stream.split();

            // reader = data coming from client (see the test below, this reader is coming from .write_allI() from the client)
            // writer = send back the same data to the client (see the test below, read_to_end is to read the data sent from server)

            // the flow is like this:
            // 1. client sends data to server using .write_all() (see test below)
            // 2. server receives it via reader in the stream (a TCP connection)
            // 3. server copy the data and writes to writer
            // 4. client reads the data sent back by the server using .read_to_end()
            tokio::io::copy(&mut reader, &mut writer).await;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_echo() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(echo(listener));

        let requests = vec!["hello", "world", "foo", "bar"];

        for request in requests {
            let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
            let (mut reader, mut writer) = socket.split();

            // Send the request to the server
            writer.write_all(request.as_bytes()).await.unwrap();
            // Close the write side of the socket
            writer.shutdown().await.unwrap();

            // Read the response
            let mut buf = Vec::with_capacity(request.len());
            reader.read_to_end(&mut buf).await.unwrap();
            assert_eq!(&buf, request.as_bytes());
        }
    }
}

use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpSocket;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>>{

    let server_addr = IpAddr::from_str("127.0.0.1")?;
    let port = 8080;
    let addr = SocketAddr::new(server_addr, port);
    let socket = TcpSocket::new_v4()?;
    let mut stream = socket.connect(addr).await?;
    let mut buff = "hello there".to_string().into_bytes();
    if let Err(e) = stream.write_all(&buff).await
    {
        eprintln!("failed to write to server: {:#}", e);
        return Ok(())
    }

    let n = match stream.read(&mut buff).await
        {
            Ok(0) =>
                {
                    println!("server closed connection");
                    return Ok(());
                },
            Ok(n) =>
                {
                    println!("size: {n}");
                    n
                },
            Err(e) =>
                {
                    eprintln!("error reading from server: {:#}", e);
                    return Ok(())
                }
        };
    let resp = String::from_utf8(buff)?;
    println!("server responded: {}", resp);

    Ok(())


}
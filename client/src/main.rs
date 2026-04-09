mod request_handler;
mod handle_response;

use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpSocket;
use crate::request_handler::send_list_request;

#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>>{

    let server_addr = IpAddr::from_str("127.0.0.1")?;
    let port = 8080;
    let addr = SocketAddr::new(server_addr, port);
    let socket = TcpSocket::new_v4()?;
    let mut stream = socket.connect(addr).await?;
    match send_list_request(&mut stream).await
    {
        Err(e) => println!("shit went south!"),
        Ok(_) => println!("ok ")
    }
    Ok(())


}
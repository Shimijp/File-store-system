mod request_handler;
mod handle_response;

use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use tokio::net::TcpSocket;
use crate::request_handler::send_list_request;
use dotenvy::dotenv;
use std::env;
#[tokio::main]
async fn main()  -> Result<(), Box<dyn Error>>{

    dotenv().ok();

    let server_addr = env::var("SERVER_HOST")?;
    let server_ip_addr = IpAddr::from_str(&*server_addr)?;
    let server_port_str = env::var("SERVER_PORT")?;

    let port = server_port_str.parse()?;
    let addr = SocketAddr::new(server_ip_addr, port);
    println!("connecting to server on: {addr}...");
    let socket = TcpSocket::new_v4()?;
    let mut stream = socket.connect(addr).await?;
    match send_list_request(&mut stream).await
    {
        Err(e) => println!("shit went south!"),
        Ok(_) => println!("ok ")
    }
    Ok(())


}
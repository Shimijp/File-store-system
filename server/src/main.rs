mod request_handler;
mod disk_handler;

use protocol::header;
use std::error::Error;
use std::net::{SocketAddr};
use tokio::io::{AsyncReadExt};
use tokio::net::{TcpListener};
use dotenvy::dotenv;
use std::env;
use crate::request_handler::{handle_header};

#[tokio::main]
async fn main() ->Result<(), Box<dyn Error>>
{

    dotenv().ok();
    let server_addr = env::var("SERVER_HOST")?;
    let server_port_str = env::var("SERVER_PORT")?;
    let server_port: u16 = server_port_str.parse()?;
        
    let addr =  format!("{server_addr}:{server_port}");
    let addr = addr.parse::<SocketAddr>()?;
    let listener = TcpListener::bind(addr).await?;
    println!("connecting on : {addr}");
    loop {
        let (mut socket ,client)=  listener.accept().await?;
        tokio::spawn(async move
            {
                let mut header_buff = [0;header::REQUEST_HEADER_SIZE];
                loop {
                    let n = match socket.read_exact(&mut header_buff ).await {
                        Ok(0) => return,
                        Ok(n) => println!("have read: {} bytes from: {}", n,client ),
                        Err(e) =>
                            {
                                eprintln!("error reading from socket: {:#}", e);
                                return;
                            }
                    };
                    match handle_header(&mut socket, &header_buff).await
                    {
                        Ok(()) => {},
                        Err(e) =>
                            {
                                println!("error handling client: {client}");
                                break
                            }
                    }



                }


            }
        );
    }


}

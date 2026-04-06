mod request_handler;
mod disk_handler;

use protocol::header;
use std::error::Error;
use std::net::{SocketAddr, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpSocket};
use crate::disk_handler::get_file_lst;

#[tokio::main]
async fn main() ->Result<(), Box<dyn Error>>
{

    let addr =  "127.0.0.1:8080";
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

                }


            }
        );
    }


}

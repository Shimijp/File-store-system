mod request_handler;
mod disk_handler;


use std::error::Error;
use std::net::{SocketAddr};
use tokio::net::{TcpListener};
use dotenvy::dotenv;
use std::env;
use crate::request_handler::{ request_handler};

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
                loop {
                    request_handler(&mut socket).await
                        .expect("failed handling client");


                }


            }
        );
    }


}

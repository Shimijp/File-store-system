mod request_handler;
mod handle_response;

use std::error::Error;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use tokio::net::TcpSocket;
use crate::request_handler::{send_list_request, send_upload_request};
use dotenvy::dotenv;
use std::{env, io};
use std::path::Path;

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
    loop {

        let mut input = String::new();
        println!("what would you like to do:\n1) request file list\n2) upload a file to server\nexit to exit");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let clean_input = input.trim();
        match clean_input
        {
            "1" => send_list_request(&mut stream)
                .await.expect("failed to send request"),
            "2" =>
                {
                    let file_path = String::new();
                    println!("enter full file path please");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    let clean_path = file_path.trim();
                    let path = Path::new(clean_path);
                    send_upload_request(path, &mut stream).await
                        .expect("failed to upload file")

                },
            "exit" => break,
            _ => {}
        };
    }
    Ok(())


}
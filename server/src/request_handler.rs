use std::mem;
use std::path::Component::ParentDir;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpSocket, TcpStream};
use protocol::header::{ResponseHeader, StatusCode, REQUEST_HEADER_SIZE};
use protocol::request::{ListReq, Request};
use protocol::response::{ListResp, Response};
use protocol::utils::{VERSION, MAGIC, ErrorCode};
use protocol::utils::ErrorCode::{ErrorConnection, ErrorIo};
use crate::disk_handler::get_file_lst;

pub fn handle_header(header: [u8;REQUEST_HEADER_SIZE])
{

}

pub async fn handle_lst_request(socket: &mut TcpStream) ->Result<(), ErrorCode>
{

   let lst = get_file_lst("C:/Users/shimo/Desktop/data_pirateGame_windows_x86_64").await.unwrap();
   for item in lst.iter(){
      println!("{item}");
   }
   let len =  lst.len() as u32;
   let payload_len = lst.iter()
       .map(|entry| 2 + entry.name.len() + 8) // 2 (u16) + string bytes + 8 (u64)
       .sum::<usize>() as u64;


   let resp =Response::<ListResp>::new(lst,len, payload_len );
   let resp_bytes: Vec<u8> = resp.try_into()?;
   match socket.write_all(&resp_bytes).await
   {
      Ok(()) => Ok(()),
      Err(e) => Err(ErrorConnection)
   }
   
   


}


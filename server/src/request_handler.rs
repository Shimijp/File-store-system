
use tokio::io::AsyncWriteExt;
use tokio::net::{ TcpStream};
use protocol::header::{Opcode, RequestHeader, REQUEST_HEADER_SIZE};
use protocol::response::{ListResp, Response};
use protocol::utils::{ ErrorCode};
use protocol::utils::ErrorCode::{ErrorConnection, ErrorIo};
use lazy_static::lazy_static;

use crate::disk_handler::get_file_lst;
const PATH_ENV : &str= "FILE_PATH";
lazy_static!
{
   static ref PATH : String =
   {
      get_path()
   };
}
pub fn get_path()->String
{
   std::env::var(PATH_ENV).expect("PATH must be set!")
}
pub async fn handle_header(stream: &mut TcpStream, header_bytes: &[u8;REQUEST_HEADER_SIZE]) -> Result<(),ErrorCode>
{

   let header = RequestHeader::try_from(header_bytes)?;
   match header.get_opcode()
   {
      Opcode::LIST => handle_lst_request(stream).await?,
      _ =>
         {
            println!("unimplemented!")

         }
   }
   Ok(())
}

pub async fn handle_lst_request(stream: &mut TcpStream) ->Result<(), ErrorCode>
{

   let path = &PATH;
   let lst = get_file_lst(path).await.map_err(|_| ErrorIo)?;
   let len =  lst.len() as u32;
   let payload_len = lst.iter()
       .map(|entry| 2 + entry.name.len() + 8) // 2 (u16) + string bytes + 8 (u64)
       .sum::<usize>() as u64;


   let resp =Response::<ListResp>::new(lst,len, payload_len );
   let resp_bytes: Vec<u8> = resp.try_into()?;
   match stream.write_all(&resp_bytes).await
   {
      Ok(()) => Ok(()),
      Err(e) =>
         {
            println!("error sending to client: {e}");
            Err(ErrorConnection)
         }
   }
   
   


}


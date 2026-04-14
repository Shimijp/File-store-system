use std::cmp::min;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{ TcpStream};
use protocol::header::{Opcode, RequestHeader, REQUEST_HEADER_SIZE};
use protocol::response::{DownloadResp, ErrorResp, ListResp, Response, UploadResp};
use protocol::utils::{ErrorCode, MAX_CHUNK_SIZE};
use protocol::utils::ErrorCode::{ErrorBadRequest, ErrorConnection, ErrorIo};
use protocol::request::{DownloadReq, UploadReq};
use crate::disk_handler::{creat_new_file, get_file_lst, open_file, PATH};
use indicatif::ProgressBar;


pub async fn request_handler(stream: &mut TcpStream) -> Result<(), ErrorCode>
{
   let mut  req_bytes = [0u8;REQUEST_HEADER_SIZE];
   stream.read_exact(&mut req_bytes).await
       .map_err(|_| ErrorConnection)?;

   match handle_header(stream, &req_bytes).await
   {
      Ok(()) => Ok(()),
      Err(e) =>
         {
            println!("{e}");

            let err_resp = Response::<ErrorResp>::new(e);
            let resp_bytes: Vec<u8>= Response::try_into(err_resp)?;

            stream.write(&resp_bytes).await
                .map_err(|_| ErrorConnection)?;
            Ok(())
         }
   }

}

pub async fn handle_header(stream: &mut TcpStream, header_bytes: &[u8;REQUEST_HEADER_SIZE]) -> Result<(),ErrorCode>
{

   let header = RequestHeader::try_from(header_bytes)?;
   match header.get_opcode()
   {
      Opcode::LIST => handle_lst_request(stream).await?,
      Opcode::UPLOAD => handle_upload_request(&header, stream).await?,
        Opcode::DOWNLOAD => handle_download_request(&header, stream).await?,
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

pub async fn handle_upload_request(request_header: &RequestHeader, stream : &mut TcpStream) ->Result<(), ErrorCode>
{
   let payload_size = request_header.get_payload_len();
   if payload_size == 0
   {
      return Err(ErrorBadRequest)
   }
   let mut data_buff = vec![0u8;min(payload_size as usize, MAX_CHUNK_SIZE)];
   let n  = stream.read(&mut data_buff).await
       .map_err(|_| ErrorConnection)?;

   if n == 0 {return Err(ErrorBadRequest)}
   let request = UploadReq::try_from(&data_buff)?;

   let filename = request.get_file_name();
   let mut file = creat_new_file(&filename).await?;

   let first_chunk = request.get_chunk();
   file.write_all(first_chunk).await
       .map_err(|_| ErrorIo)?;


   let mut reminder = payload_size - first_chunk.len() as u64;
   let bar = ProgressBar::new(reminder);
   let mut buffer = vec![0u8; min(MAX_CHUNK_SIZE, reminder as usize)];
   while reminder > 0 {
      let n = stream.read(&mut buffer).await
          .map_err(|_| ErrorConnection)?;
      if n == 0 {break};
      file.write_all(&buffer[..n]).await
          .map_err(|_| ErrorIo)?;
      reminder -= n as u64;
      bar.inc(n as u64);

   }
   bar.finish();

   let resp = Response::<UploadResp>::new();
   let resp_bytes: Vec<u8> = resp.try_into()?;
   stream.write_all(&resp_bytes).await
       .map_err(|_| ErrorConnection)?;

   Ok(())
}
pub async fn handle_download_request(request_header: &RequestHeader, stream : &mut TcpStream) ->Result<(), ErrorCode>
{
   let name_len  = request_header.get_filename_len() as usize;
    let mut data_buff = vec![0u8;name_len + 2];
    stream.read_exact(&mut data_buff).await
        .map_err(|_| ErrorConnection)?;
    let request = DownloadReq::try_from(&data_buff)?;
    let file_name = request.get_file_name();

    let mut file = open_file(&file_name).await?;
    let metadata = file.metadata().await
        .map_err(|_| ErrorIo)?;
    let file_size = metadata.len();
    let mut buffer = vec![0u8; min(MAX_CHUNK_SIZE, file_size as usize)];
    let mut reminder = file_size;
    // Send only header first; file bytes are streamed below.
    let resp = Response::<DownloadResp>::new(file_size, Vec::new());
    let resp_bytes: Vec<u8> = resp.try_into()?;
    stream.write_all(&resp_bytes).await
        .map_err(|_| ErrorConnection)?;

    while reminder > 0 {
        let n = file.read(&mut buffer).await
            .map_err(|_| ErrorIo)?;
        if n == 0 { break }
        stream.write_all(&buffer[..n]).await
            .map_err(|_| ErrorConnection)?;
       reminder -= n as u64;
    }

   Ok(())
}


use std::cmp::{ min};
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use protocol::header::{ ResponseHeader, StatusCode, RESPONSE_HEADER_SIZE};
use protocol::request::{DownloadReq, ListReq, Request, UploadReq};
use protocol::utils::{ErrorCode, MAX_CHUNK_SIZE};
use protocol::utils::ErrorCode::{ErrorConnection, ErrorIo, ErrorNotFound};
use indicatif::ProgressBar;

use crate::handle_response::{handle_download_response, handle_lst_response, handle_upload_response};

pub async fn send_list_request(stream: &mut TcpStream) ->Result<(), ErrorCode>
{
    let list_req = Request::<ListReq>::new();
    let mut  buffer : Vec<u8> = list_req.try_into()?;
    let mut resp_header_buff = [0;RESPONSE_HEADER_SIZE];
    if let Err(e) = stream.write_all(&mut buffer).await
    {
        println!("failed to send to server!: {e}");
        return Err(ErrorConnection)
    }

    match stream.read_exact(&mut resp_header_buff).await
    {
        Ok(0) =>
            {
                println!("server closed connection!");
                 return Err(ErrorConnection)
            }
        Ok(n) => println!("received {n} bytes from server"),
        Err(e) =>
            {
                eprintln!("error occurred : {:?}", e);
                return Err(ErrorCode::UnknownErr)
            }
    }
    let resp_header = ResponseHeader::try_from(&resp_header_buff)?;

    match resp_header.get_status_code()
    {
        StatusCode::Ok => handle_lst_response(stream, & resp_header).await?,
        error =>
            {
                eprintln!("server responded with error: {:?}", error);
                return Err(ErrorCode::UnknownErr)
            }
    }
    Ok(())
}

pub async fn send_upload_request(path  : &Path,stream :&mut TcpStream)->Result<(), ErrorCode>
{
    let mut file = File::open(path).await
        .map_err(|_| ErrorNotFound)?;
    let metadata = file.metadata().await
        .map_err(|_| ErrorIo)?;
    let file_name = path.file_name()
        .ok_or(ErrorNotFound)?;
    let file_name_str = file_name.to_str()
        .ok_or(ErrorNotFound)?;

    //windows specific
    let payload_size = metadata.file_size();

    let name_len = file_name_str.len() as u16;
    let mut first_chunk = vec![0u8; min(MAX_CHUNK_SIZE, payload_size as usize)];

    let n = file.read(&mut first_chunk).await
        .map_err(|_| ErrorIo)?;


    let request = Request::<UploadReq>::new(file_name_str.to_string(), name_len,payload_size , first_chunk);
    let req_bytes :Vec<u8> = request.try_into()?;
    stream.write_all(&req_bytes).await
        .map_err(|_| ErrorConnection)?;
    let mut  reminder = payload_size - n as u64;
    let bar = ProgressBar::new(reminder);
    let mut buffer = vec![0u8;min(reminder as usize, MAX_CHUNK_SIZE )];
    while reminder > 0 {
        let n =  file.read(&mut buffer).await
            .map_err(|_| ErrorIo)?;
        if n == 0 {break}

        stream.write_all(&buffer[..n]).await
            .map_err(|_| ErrorConnection)?;
        reminder -=n as u64;
        bar.inc(n  as u64);
    }
    bar.finish();
    let mut resp_buff = [0u8;RESPONSE_HEADER_SIZE];
    stream.read_exact(&mut resp_buff).await
        .map_err(|_| ErrorConnection)?;
    let resp_header = ResponseHeader::try_from(&resp_buff)?;

    handle_upload_response(&resp_header)
}
pub async fn send_download_request(file_name : &str,  stream: &mut TcpStream) ->Result<(), ErrorCode>
{
    let name_len = file_name.len() as u16;
    let request = Request::<DownloadReq>::new(file_name.to_string(), name_len);
    let req_bytes :Vec<u8> = request.try_into()?;
    stream.write_all(&req_bytes).await
        .map_err(|_| ErrorConnection)?;
    let mut resp_buff = [0u8;RESPONSE_HEADER_SIZE];
    stream.read_exact(&mut resp_buff).await
        .map_err(|_| ErrorConnection)?;
    let resp_header = ResponseHeader::try_from(&resp_buff)?;
    handle_download_response(&resp_header, stream, file_name).await?;
    Ok(())

}
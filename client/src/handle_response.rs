use std::cmp::min;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use protocol::header::{ResponseHeader, StatusCode};
use protocol::response::{ ListResp};
use protocol::utils::{ErrorCode, MAX_CHUNK_SIZE};
use protocol::utils::ErrorCode::ErrorConnection;
use crate::client_dsk_handler::creat_new_file;

pub async fn handle_lst_response(stream : &mut TcpStream, resp_header: &ResponseHeader) ->Result<(), ErrorCode>
{
    println!("expecting {} from server", resp_header.get_payload_len());
    let mut resp_buff = vec![0u8;resp_header.get_payload_len() as usize];
    stream.read_exact(&mut resp_buff).await
        .map_err(|_| ErrorConnection)?;



    match resp_header.get_status_code()
    {
        StatusCode::Ok => handle_list_response(&resp_buff),
        e =>
            {
                println!("server responded with error: {:?}", e);
                Err(ErrorCode::UnknownErr)
            }
    }

}

pub fn handle_list_response(buffer: &Vec<u8>) ->Result<() , ErrorCode>
{
    let list_resp = ListResp::try_from(buffer)?;
    println!("{list_resp}");
    Ok(())
}
pub  fn handle_upload_response(resp_header: &ResponseHeader) ->Result<(), ErrorCode>
{
    match resp_header.get_status_code()
    {
        StatusCode::Ok =>
            {
                println!("upload successfully!");
                Ok(())
            }
        e =>
            {
                println!("server responded with error: {:?}", e);
                Err(ErrorCode::UnknownErr)
            }

    }
}
pub async fn  handle_download_response(resp_header: &ResponseHeader , stream : &mut TcpStream, filename :&str) ->Result<(), ErrorCode>
{
    match resp_header.get_status_code()
    {
        StatusCode::Ok =>
            {
                println!("preparing to receive file from server...");

            }
        e =>
            {
                println!("server responded with error: {:?}", e);
                return Err(ErrorCode::UnknownErr)
            }

    }
    let file_size = resp_header.get_payload_len();
    let mut filebuff = vec![0u8; min(file_size as usize, MAX_CHUNK_SIZE)];
    let mut file = creat_new_file(&filename).await?;
    let mut reminder = file_size;
    while reminder > 0 {
        let n = stream.read(&mut filebuff).await
            .map_err(|_| ErrorConnection)?;
        if n == 0 { break }
        file.write(&filebuff[..n]).await
            .map_err(|_| ErrorCode::ErrorIo)?;
        reminder -= n as u64;
    }
    Ok(())
}


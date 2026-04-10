use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use protocol::header::{ResponseHeader, StatusCode};
use protocol::response::ListResp;
use protocol::utils::ErrorCode;
use protocol::utils::ErrorCode::ErrorConnection;

pub async fn handle_lst_response(stream : &mut TcpStream, resp_header: &ResponseHeader) ->Result<(), ErrorCode>
{
    println!("expecting {} from server", resp_header.get_payload_len());
    let mut resp_buff = vec![0u8;resp_header.get_payload_len() as usize];
    let n =  stream.read_exact(&mut resp_buff).await
        .map_err(|_| ErrorConnection)?;



    //todo : better error handling, need to implement display for error
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
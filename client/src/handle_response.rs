use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use protocol::header::ResponseHeader;
use protocol::response::ListResp;
use protocol::utils::ErrorCode;
use protocol::utils::ErrorCode::ErrorConnection;

pub async fn handle_response_header(stream : &mut TcpStream, resp_header: &ResponseHeader) ->Result<(), ErrorCode>
{
    println!("expecting {} from server", resp_header.get_payload_len());
    let mut resp_buff = vec![0u8;resp_header.get_payload_len() as usize];
    let n =  stream.read_exact(&mut resp_buff).await
        .map_err(|_| ErrorConnection)?;

    match resp_header.get_status_code()
    {
        ListResp => return handle_list_response(&resp_buff)
    }
    Ok(())
}

pub fn handle_list_response(buffer: &Vec<u8>) ->Result<() , ErrorCode>
{
    let list_resp = ListResp::try_from(buffer)?;
    println!("{list_resp}");
    Ok(())
}
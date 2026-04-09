use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use protocol::header::{ResponseHeader, StatusCode, RESPONSE_HEADER_SIZE};
use protocol::request::{ListReq, Request};
use protocol::response::{ErrorResp, ListResp, Response};
use protocol::utils::ErrorCode;
use protocol::utils::ErrorCode::ErrorConnection;

pub async fn send_list_request(stream: &mut TcpStream) ->Result<ListResp, ErrorCode>
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
        StatusCode::Ok => {},
        error =>
            {
                eprintln!("server responded with error: {:?}", error);
                return Err(ErrorCode::UnknownErr)
            }
    }
    println!("expecting {} from server", resp_header.get_payload_len());
    let mut resp_buff = vec![0u8;resp_header.get_payload_len() as usize];
    let n = match stream.read_exact(&mut resp_buff).await
    {
        Ok(0) =>
            {
                println!("server closed connection!");
                return Err(ErrorConnection)
            },
        Ok(n) => println!("received {n} bytes from server"),
        Err(e) =>
            {
                eprintln!("error occurred : {:?}", e);
                return Err(ErrorCode::UnknownErr)
            }

    };
    let list_resp = ListResp::try_from(&resp_buff)?;
    Ok(list_resp)






}
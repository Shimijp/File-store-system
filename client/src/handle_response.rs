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
    println!("{list_resp}");
    Ok(())
}
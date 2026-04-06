use protocol::header::{ResponseHeader, StatusCode, REQUEST_HEADER_SIZE};
use protocol::utils::{VERSION, MAGIC};
use crate::disk_handler::get_file_lst;

pub fn handle_header(header: [u8;REQUEST_HEADER_SIZE])
{

}

pub async fn handle_lst_request() -> Vec<u8>
{
   Vec::new()
}


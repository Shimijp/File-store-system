use crate::header::StatusCode;
use crate::utils::ErrorCode::ErrorExists;

pub const MAGIC: u16 = (0x46 << 8) | (0x53);
pub const VERSION: u8 = 0x1;
pub const MAX_CHUNK_SIZE: usize = 64 * 2^10;  //64KB
#[derive(Debug)]
pub enum ErrorCode
{
    UnknownErr,
    ErrorNotFound,
    ErrorIo,
    ErrorExists,
    ErrorBadRequest,
    ErrorBadResponse,
    ErrorTimeOut,
    ErrorConnection,
    Empty // not actual error, just for when a function doesn't need to return anything and i cant use option

}
impl ErrorCode
{
    pub fn map_to_stat_error(self)-> StatusCode
    {
        let status_code = match self
        {
            ErrorCode::ErrorIo => StatusCode::ErrorIo,
            ErrorCode::ErrorBadRequest => StatusCode::ErrorBadRequest,
            ErrorCode::ErrorNotFound => StatusCode::ErrorNotFound,
            ErrorExists => StatusCode::ErrorExists,
            _ => StatusCode::ErrorBadRequest

        };
        status_code
    }
}
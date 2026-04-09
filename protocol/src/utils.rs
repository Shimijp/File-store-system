use std::fmt;
use std::fmt::{Display, Formatter};
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
    ErrorMem,
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
impl Display for ErrorCode
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::UnknownErr => write!(f, "Unknown error occurred"),
            ErrorCode::ErrorNotFound => write!(f, "File or resource not found"),
            ErrorCode::ErrorIo => write!(f, "I/O error occurred"),
            ErrorCode::ErrorExists => write!(f, "File or resource already exists"),
            ErrorCode::ErrorBadRequest => write!(f, "Bad request format"),
            ErrorCode::ErrorBadResponse => write!(f, "Bad response format"),
            ErrorCode::ErrorTimeOut => write!(f, "Operation timed out"),
            ErrorCode::ErrorMem => write!(f, "Memory error"),
            ErrorCode::ErrorConnection => write!(f, "Connection error"),
            ErrorCode::Empty => write!(f, "Empty operation"),
        }
    }
}

impl std::error::Error for ErrorCode {}
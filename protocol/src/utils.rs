pub const MAGIC: u16 = (0x46 << 8) | (0x53);
pub const VERSION: u8 = 0x1;
pub const MAX_CHUNK_SIZE: usize = 64 * 2^10;  //64KB
pub enum ErrorCode
{
    UnknownErr,
    ErrorNotFound,
    ErrorIo,
    ErrorExists,
    ErrorBadRequest,
    ErrorBadResponse,
    ErrorTimeOut,
    Empty // not actual error, just for when a function doesn't need to return anything and i cant use option

}
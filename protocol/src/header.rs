use crate::utils;

pub const REQUEST_HEADER_SIZE :usize = 14;
pub const RESPONSE_HEADER_SIZE : usize = 12;
#[derive(Debug, Clone, Copy)]
pub enum Opcode
{
    LIST = 0x1,
    UPLOAD = 0x2,
    DOWNLOAD = 0x3,
    DELETE = 0x4

}


#[derive(Debug, Clone, Copy)]
pub enum StatusCode
{
    Ok = 0x0,
    ErrorNotFound,
    ErrorExists,
    ErrorIo,
    ErrorBadRequest = 0xFF


}
pub enum Error
{
    BadRequest,
    BadResponse

}
struct RequestHeader
{
    magic : u16 ,
    version :u8,
    opcode: Opcode,
    filename_len : u16,
    payload_len : u64,
}
pub struct ResponseHeader
{
    magic : u16,
    version : u8 ,
    status_code : StatusCode ,
    payload_len : u64,
}
impl RequestHeader
{
    pub fn new( opcode: Opcode, filename_len: u16, payload_len: u64)->Self
    {
        RequestHeader
        {
            magic : utils::MAGIC,
            version : utils::VERSION,
            opcode,
            filename_len,
            payload_len
        }
    }
}

impl ResponseHeader
{
    pub fn new(status_code: StatusCode,payload_len : u64) ->Self
    {
        ResponseHeader
        {
            magic : utils::MAGIC,
            version : utils::VERSION,
            status_code,
            payload_len
        }
    }
}
impl Into<[u8;RESPONSE_HEADER_SIZE]> for ResponseHeader
{
    fn into(self) -> [u8;RESPONSE_HEADER_SIZE]
    {
        let mut buffer = [0;RESPONSE_HEADER_SIZE];
        let magic_chunks = self.magic.to_be_bytes();
        let payload_chunks = self.payload_len.to_be_bytes();
        buffer[..2].copy_from_slice(&magic_chunks);
        buffer[2] = self.version;
        buffer[3] = self.status_code as u8;
        buffer[4..].copy_from_slice(&payload_chunks);
        buffer

    }
}
impl Into<[u8;REQUEST_HEADER_SIZE]> for RequestHeader
{
    fn into(self) -> [u8; REQUEST_HEADER_SIZE] {
        let mut buffer  = [0;REQUEST_HEADER_SIZE];
        let magic_chunks = self.magic.to_be_bytes();
        let filename_bytes = self.filename_len.to_be_bytes();
        let payload_chunks: [u8;8] = self.payload_len.to_be_bytes();
        buffer[..2].copy_from_slice(&magic_chunks);
        buffer[2] = self.version;
        buffer[3] = self.opcode as u8;
        buffer[4..6].copy_from_slice(&filename_bytes);
        buffer[6..].copy_from_slice(&payload_chunks);
        buffer
    }
}

impl TryFrom<&[u8;REQUEST_HEADER_SIZE]> for RequestHeader
{
    type Error = Error;

    fn try_from(buffer: &[u8;REQUEST_HEADER_SIZE]) -> Result<Self, Self::Error> {
        let magic = u16::from_be_bytes(buffer[..2].try_into().unwrap());
        if magic != utils::MAGIC {
            return Err(Error::BadRequest)
        }
        let version = buffer[2];
        let opcode_byte = buffer[3];
        let filename_len = u16::from_be_bytes(buffer[4..6].try_into().unwrap());
        let payload_len = u64::from_be_bytes(buffer[7 .. REQUEST_HEADER_SIZE].try_into().unwrap());

        let opcode = match opcode_byte {
            0x1 => Opcode::LIST,
            0x2 => Opcode::UPLOAD,
            0x3 => Opcode::DOWNLOAD,
            0x4 => Opcode::DELETE,
            _ =>
                {
                    return Err(Error::BadRequest)
                }
        };
        Ok(
            RequestHeader
            {
                magic ,
                version,
                opcode,
                filename_len ,
                payload_len
            }
        )
    }
}

impl TryFrom<&[u8;REQUEST_HEADER_SIZE]> for ResponseHeader
{
    type Error = Error;

    fn try_from(buffer: &[u8; REQUEST_HEADER_SIZE]) -> Result<Self, Self::Error> {
        let magic = u16::from_be_bytes(buffer[..2].try_into().unwrap());
        if magic != utils::MAGIC {
            return Err(Error::BadRequest)
        }
        let version = buffer[2];
        let status_code_byte = buffer[3];
        let payload_len = u64::from_be_bytes(buffer[4..].try_into().unwrap());
        let status_code = match status_code_byte  {
            0x00 => StatusCode::Ok,
            0x01 => StatusCode::ErrorNotFound,
            0x02 => StatusCode::ErrorExists,
            0x03 => StatusCode::ErrorIo,
            0xFF => StatusCode::ErrorBadRequest,
            _ => return Err(Error::BadResponse)


        };
        Ok(ResponseHeader
        {
            magic,
            version,
            status_code,
            payload_len
        })

    }
}
use crate::header::{RequestHeader, Opcode, REQUEST_HEADER_SIZE};
use crate::utils::{ErrorCode, MAGIC, MAX_CHUNK_SIZE, VERSION};
use crate::utils::ErrorCode::{ErrorBadRequest, UnknownErr};

type ListRequest = Request<ListReq>;
type DownloadRequest = Request<DownloadReq>;
type DeleteRequest = Request<DeleteReq>;

pub struct Request<T>
where
    T : TryInto<Vec<u8>>
{
    header: RequestHeader,
    content : T
}
pub struct ListReq;

pub struct DownloadReq
{
    filename_len : u16,
    filename : String
}
pub struct DeleteReq
{
    filename_len : u16,
    filename : String
}

pub struct UploadReq
{
    filename_len : u16,
    filename : String,
    payload_size : u64,
    chunk : Vec<u8>
}

impl<T> TryInto<Vec<u8>> for Request<T>
where
    T: TryInto<Vec<u8>, Error = ErrorCode>
{
    type Error = ErrorCode;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut buffer = Vec::new();
        let header_bytes : [u8;REQUEST_HEADER_SIZE] = self.header.into();

        let body_bytes = match self.content.try_into()
        {
            Ok(bytes) => bytes,
            Err(ErrorCode::Empty) => Vec::new(),
            Err(_) => return Err(ErrorBadRequest)


        };
        buffer.extend(header_bytes);
        buffer.extend(body_bytes);

        Ok(buffer)
    }
}

impl TryInto<Vec<u8>> for ListReq
{
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Err(ErrorCode::Empty)
    }
}

impl TryInto<Vec<u8>> for DownloadReq
{
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        if self.filename_len as usize != self.filename.len()
        {
            return Err(UnknownErr)
        }
        let mut buffer = Vec::new();
        let filename_len_bytes = self.filename_len.to_be_bytes();
        let filename_bytes = self.filename.into_bytes();
        buffer.extend(filename_len_bytes);
        buffer.extend(filename_bytes);

        Ok(buffer)
    }
}

impl TryInto<Vec<u8>> for DeleteReq
{
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        if self.filename_len as usize != self.filename.len()
        {
            return Err(UnknownErr)
        }
        let mut buffer = Vec::new();
        let filename_len_bytes = self.filename_len.to_be_bytes();
        let filename_bytes = self.filename.into_bytes();
        buffer.extend(filename_len_bytes);
        buffer.extend(filename_bytes);

        Ok(buffer)
    }
}

impl TryInto<Vec<u8>> for UploadReq
{
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        if self.filename_len as usize != self.filename.len()
        {
            return Err(UnknownErr)
        }
        if (self.payload_size as usize) < self.chunk.len() || self.chunk.len() > MAX_CHUNK_SIZE
        {
            return Err(ErrorBadRequest)
        }
        let mut buffer = Vec::new();
        let filename_len_bytes = self.filename_len.to_be_bytes();
        let filename_bytes = self.filename.into_bytes();
        let payload_len_bytes = self.payload_size.to_be_bytes();
        buffer.extend(filename_len_bytes);
        buffer.extend(filename_bytes);
        buffer.extend(payload_len_bytes);
        buffer.extend(self.chunk);

        Ok(buffer)
    }
}

impl Request<ListReq>
{
    pub fn new() -> Self
    {
        Request
        {
            header : RequestHeader::new(Opcode::LIST, 0, 0),
            content : ListReq
        }
    }
}

impl Request<DownloadReq>
{
    pub fn new(filename: String, filename_len: u16) -> Self
    {
        Request{
            header : RequestHeader::new(
                Opcode::DOWNLOAD, filename_len, 0
            ),
            content : DownloadReq{
                filename_len,
                filename
            }
        }
    }
}

impl Request<DeleteReq>
{
    pub fn new(filename: String, filename_len: u16) -> Self
    {
        Request{
            header : RequestHeader::new(
                Opcode::DELETE, filename_len, 0
            ),
            content : DeleteReq{
                filename_len,
                filename
            }
        }
    }
}

impl Request<UploadReq>
{
    pub fn new(filename: String, filename_len: u16, payload_size: u64, chunk: Vec<u8>) -> Self
    {
        Request{
            header : RequestHeader::new(
                Opcode::UPLOAD, filename_len, payload_size
            ),
            content : UploadReq{
                filename_len,
                filename,
                payload_size,
                chunk
            }
        }
    }
}
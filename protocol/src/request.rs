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

impl UploadReq {
    pub fn get_file_name(&self) -> &str{
         self.filename.as_str()
    }
    pub fn get_chunk(&self) -> &[u8]{
        &self.chunk
    }
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
impl TryFrom<&Vec<u8>> for UploadReq
{
    type Error = ErrorCode;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {

        if value.len() < 2{
            return Err(ErrorBadRequest)
        };
        let file_name_len = u16::from_be_bytes(value[0..2].try_into().unwrap());
        let name_start: usize = 2;
        let name_end = name_start + file_name_len as usize;
        let payload_size_end = name_end + 8;

        let filename_bytes: &_ = value
            .get(name_start..name_end)
            .ok_or(ErrorBadRequest)?;
        let name = String::from_utf8(filename_bytes.to_vec())
            .map_err(|_| ErrorBadRequest)?;

        let payload_size_bytes = value
            .get(name_end..payload_size_end)
            .ok_or(ErrorBadRequest)?;

        let payload_size: u64 = u64::from_be_bytes(payload_size_bytes.try_into().unwrap());
        let chunk_size =  value.len()  - (2 + name.len() + 8);
        if chunk_size > MAX_CHUNK_SIZE {return  Err(ErrorBadRequest)}
        let chunk = value
            .get(payload_size_end .. payload_size_end + chunk_size)
            .ok_or(ErrorBadRequest)?;
        Ok(UploadReq
        {
            filename_len: name.len() as u16,
            filename: name,
            payload_size,
            chunk: chunk.to_vec()
        })




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
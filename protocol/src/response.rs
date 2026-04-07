use std::collections::hash_map::Entry;
use crate::header::{ResponseHeader, StatusCode, RESPONSE_HEADER_SIZE};
use crate::utils::{ErrorCode, MAGIC, MAX_CHUNK_SIZE, VERSION};
use crate::utils::ErrorCode::{ErrorBadResponse, UnknownErr};

type UploadResponse = Response<UploadResp>;
type ListResponse = Response<ListResp>;
type DeleteResponse = Response<DeleteResp>;
type ErrorResponse = Response<ErrorResp>;

pub struct Response<T >
where
    T : TryInto<Vec<u8>>
{
    header: ResponseHeader,
    content : T
}
pub struct ListResp
{
    file_count : u32,
    entries: Vec<FileEntry>

}
pub struct UploadResp;
pub struct DeleteResp;

struct ErrorResp
{
    status_code: StatusCode
}
struct DownloadResp
{
    payload_size : u64,
    chunk : Vec<u8>
}
struct FileEntry
{
    name_len:u16,
    name : String,
    file_size :  u64
}

impl<T> TryInto<Vec<u8>> for Response<T>
where
    T: TryInto<Vec<u8>, Error = ErrorCode>
{
    type Error = ErrorCode;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut buffer = Vec::new();
        let header_bytes : [u8;RESPONSE_HEADER_SIZE] = self.header.into();
        let body_bytes = match self.content.try_into()
        {
            Ok(bytes) => bytes,
            Err(ErrorCode::Empty) => Vec::new(),
            Err(_) => return Err(ErrorBadResponse)


        };
        buffer.extend(header_bytes);
        buffer.extend(body_bytes);

        Ok(buffer)
    }
}

impl TryInto<Vec<u8>> for UploadResp
{
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Err(ErrorCode::Empty)
    }
}
impl TryInto<Vec<u8>> for DeleteResp
{
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Err(ErrorCode::Empty)

    }
}

impl  TryInto<Vec<u8>> for ErrorResp
{
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(vec![self.status_code as u8])
    }

}
impl TryInto<Vec<u8>> for FileEntry
{
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        if self.name_len as usize != self.name.len()
        {
            return Err(UnknownErr)
        }
        let mut entry_buff = Vec::new();
        let mem_len_high = self.name_len.to_be_bytes();
        let name_bytes = self.name.into_bytes();
        let file_size_bytes = self.file_size.to_be_bytes();
        entry_buff.extend(mem_len_high);
        entry_buff.extend(name_bytes);
        entry_buff.extend(file_size_bytes);






        Ok(entry_buff)
    }
}
impl TryInto<Vec<u8>> for ListResp
{
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut buffer = Vec::new();
        for entry in self.entries.into_iter(){
            let entry_buff: Vec<u8> = entry.try_into()?;
            buffer.extend(entry_buff);
        }
        Ok(buffer)
    }
}
impl TryInto<Vec<u8>> for DownloadResp
{
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {

        if (self.payload_size as usize) < self.chunk.len() || self.chunk.len() > MAX_CHUNK_SIZE
        {
            return Err(ErrorBadResponse)
        }
        let mut buffer = Vec::new();
        let payload_len_bytes = self.payload_size.to_be_bytes();
        buffer.extend(payload_len_bytes);
        buffer.extend(self.chunk);

        Ok(buffer)
    }
}
impl Response<UploadResp>
{
    pub fn new() -> Self
    {
        Response
        {
            header : ResponseHeader::new(StatusCode::Ok,0),
            content : UploadResp
        }
    }
}

impl Response<DownloadResp>
{
    fn new(payload_size: u64, chunk : Vec<u8>) ->Self
    {
        Response{
            header : ResponseHeader::new(
                StatusCode::Ok, payload_size
            ),
            content : DownloadResp{
                payload_size,
                chunk
            }
        }
    }
}
impl Response<ListResp>
{
    fn new(list: Vec<FileEntry>, file_count: u32, size : u64) ->Self
    {
        Response{
            header : ResponseHeader::new(StatusCode::Ok, size),
            content : ListResp
            {
                file_count,
                entries: list
            }
        }
    }
}



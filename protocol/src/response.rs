use std::fmt::{Display, Formatter};
use std::io::{Cursor, ErrorKind, Read};
use crate::header::{ResponseHeader, StatusCode, RESPONSE_HEADER_SIZE};
use crate::utils::ErrorCode::{ErrorBadResponse, UnknownErr};
use crate::utils::{ErrorCode, MAX_CHUNK_SIZE};


pub struct Response<T>
where
    T: TryInto<Vec<u8>>,
{
    header: ResponseHeader,
    content: T,
}
pub struct ListResp {
    file_count: u32,
    entries: Vec<FileEntry>,
}
pub struct UploadResp;
pub struct DeleteResp;

pub struct ErrorResp {
    status_code: StatusCode,
}
pub struct DownloadResp {
    payload_size: u64,
    chunk: Vec<u8>,
}
pub struct FileEntry {
    pub name_len: u16,
    pub name: String,
    pub file_size: u64,
}
impl FileEntry{
    pub fn new(name_len: u16, name: String , file_size: u64) -> Result<Self, ErrorCode>
    {
        if (name_len as usize) != name.len()
        {
            return Err(ErrorBadResponse)
        }
        Ok(FileEntry{
            name_len,
            name,
            file_size
        })
    }
}

impl<T> TryInto<Vec<u8>> for Response<T>
where
    T: TryInto<Vec<u8>, Error = ErrorCode>,
{
    type Error = ErrorCode;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut buffer = Vec::new();
        let header_bytes: [u8; RESPONSE_HEADER_SIZE] = self.header.into();
        let body_bytes = match self.content.try_into() {
            Ok(bytes) => bytes,
            Err(ErrorCode::Empty) => Vec::new(),
            Err(_) => return Err(ErrorBadResponse),
        };
        buffer.extend(header_bytes);
        buffer.extend(body_bytes);

        Ok(buffer)
    }
}

impl TryInto<Vec<u8>> for UploadResp {
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Err(ErrorCode::Empty)
    }
}
impl TryInto<Vec<u8>> for DeleteResp {
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Err(ErrorCode::Empty)
    }
}

impl TryInto<Vec<u8>> for ErrorResp {
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        Ok(vec![self.status_code as u8])
    }
}
impl TryInto<Vec<u8>> for FileEntry {
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        if self.name_len as usize != self.name.len() {
            return Err(UnknownErr);
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
impl TryInto<Vec<u8>> for ListResp {
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        let mut buffer = Vec::new();
        for entry in self.entries.into_iter() {
            let entry_buff: Vec<u8> = entry.try_into()?;
            buffer.extend(entry_buff);
        }
        Ok(buffer)
    }
}
impl TryInto<Vec<u8>> for DownloadResp {
    type Error = ErrorCode;
    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        if (self.payload_size as usize) < self.chunk.len() || self.chunk.len() > MAX_CHUNK_SIZE {
            return Err(ErrorBadResponse);
        }
        let mut buffer = Vec::new();
        let payload_len_bytes = self.payload_size.to_be_bytes();
        buffer.extend(payload_len_bytes);
        buffer.extend(self.chunk);

        Ok(buffer)
    }
}
impl TryFrom<Vec<u8>> for UploadResp {
    type Error = ErrorCode;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let header_bytes : [u8;RESPONSE_HEADER_SIZE] = value.try_into().unwrap();
        ResponseHeader::try_from(&header_bytes)?;
        Err(ErrorBadResponse)
    }
}
impl Response<UploadResp> {
    pub fn new() -> Self {
        Response {
            header: ResponseHeader::new(StatusCode::Ok, 0),
            content: UploadResp,
        }
    }
}

impl Response<DownloadResp> {
    pub fn new(payload_size: u64, chunk: Vec<u8>) -> Self {
        Response {
            header: ResponseHeader::new(StatusCode::Ok, payload_size),
            content: DownloadResp {
                payload_size,
                chunk,
            },
        }
    }

}
impl Response<ListResp> {
    pub fn new(list: Vec<FileEntry>, file_count: u32, size: u64) -> Self {
        Response {
            header: ResponseHeader::new(StatusCode::Ok, size),
            content: ListResp {
                file_count,
                entries: list,
            },
        }
    }
}
impl Response<ErrorResp>
{
    pub fn new(error_code: ErrorCode) -> Self
    {
        let status_code = error_code.map_to_stat_error();
        Response
        {
            header : ResponseHeader::new(status_code,0),
            content : ErrorResp
            {
                status_code
            }
        }
    }
}

impl TryFrom<&Vec<u8>> for ListResp
{
    type Error = ErrorCode;

    fn try_from(value: &Vec<u8>) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(value);
        let mut items = Vec::new();
        let mut file_count = 0u32;
        loop {
            let mut len_bytes = [0u8; 2];


            match cursor.read_exact(&mut len_bytes) {
                Ok(_) => {}
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,



                Err(_) =>
                    {
                        return Err(ErrorBadResponse)
                    },
            }

            let name_len = u16::from_be_bytes(len_bytes) as usize;

            // Read the actual data chunk
            // Note: vec![0; len] allocates memory on EVERY iteration
            let mut item_bytes = vec![0u8; name_len];
            if let Err(e) = cursor.read_exact(&mut item_bytes)
            {
                if e.kind() == ErrorKind::UnexpectedEof { break};
                return Err(ErrorBadResponse)
            }

            let item_str = match String::from_utf8(item_bytes)
            {
                Err(_) => return Err(ErrorBadResponse),
                Ok(item_str) => item_str
            };


            let mut filelen_bytes = [0u8; 8];
            if let Err(_) = cursor.read_exact(&mut filelen_bytes)
            {
                return Err(ErrorBadResponse);
            }
            let file_len = u64::from_be_bytes(filelen_bytes);


            let entry = FileEntry::new(name_len as u16, item_str, file_len).unwrap();
            items.push(entry);
            file_count += 1;
        }

        Ok(ListResp {
            file_count,
            entries: items,
        })
    }
}

impl TryFrom<Vec<u8>> for DownloadResp
{
    type Error = ErrorCode;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() < 8
        {
            return Err(ErrorBadResponse)
        }
        let payload_size = u64::from_be_bytes(value[0..8].try_into().unwrap());
        let chunk = value[8..].to_vec();
        if chunk.len() > MAX_CHUNK_SIZE || chunk.len() > payload_size as usize
        {
            return Err(ErrorBadResponse)
        }
        Ok(DownloadResp {
            payload_size,
            chunk,
        })
    }
}
impl Display for FileEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} bytes)", self.name, self.file_size)
    }
}
impl Display for ListResp
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "--- File List ({} items) ---", self.file_count)?;

        for entry in &self.entries
        {
            writeln!(f, "  - {}", entry)?;
        }
        write!(f, "----------------------------")
    }
}




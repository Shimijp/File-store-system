use crate::header::ResponseHeader;

struct Response<T  >
    where T : Into<Vec<u8>> 
{
    header: ResponseHeader,
    content : T
}
pub struct ListResp
{
    file_count : u32,
    entries: Vec<FileEntry>

}


struct FileEntry
{
    mem_len :u16,
    name : String,
    file_size :  u64


}
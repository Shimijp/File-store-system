use std::io::ErrorKind::FileTooLarge;
use tokio::{fs, io};
use tokio::io::AsyncReadExt;
use protocol::response::FileEntry;
use protocol::utils;

pub async fn get_file_lst(path: &str) -> Result<Vec<FileEntry>, io::Error>
{
    let mut entries = fs::read_dir(path).await?;
    let mut list = Vec::new();

    while let Some(entry) = entries.next_entry().await?
    {

        println!("{:?}", entry.file_name());
        match FileEntry::new(entry.file_name().len() as u16, entry.file_name().into_string().unwrap(), entry.metadata().await?.len())
        {
            Ok(file_entry) =>
                {
                    list.push(file_entry)
                },
            Err(e) =>
                {
                    println!("error : {e}");
                    return Err(io::Error::new(FileTooLarge,"this shouldn't happen"))
                }

        };



    }
    Ok(list)
}



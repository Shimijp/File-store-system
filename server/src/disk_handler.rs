use tokio::{fs, io};
use tokio::io::AsyncReadExt;
use protocol::utils;

pub async fn get_file_lst(path: &str) -> Result<Vec<String>, io::Error>
{
    let mut entries = fs::read_dir(path).await?;
    let mut list = Vec::new();
    while let Some(entry) = entries.next_entry().await?
    {
        list.push(entry.file_name().into_string().unwrap())
    }
    Ok(list)
}



use std::io::ErrorKind::FileTooLarge;
use lazy_static::lazy_static;
use tokio::{fs, io};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use protocol::response::FileEntry;
use protocol::utils;
use protocol::utils::ErrorCode;
use protocol::utils::ErrorCode::ErrorExists;

const PATH_ENV : &str= "FILE_PATH";
lazy_static!
{
   pub static ref PATH : String =
   {
      get_path()
   };
}
pub fn get_path()->String
{
    std::env::var(PATH_ENV).expect("PATH must be set!")
}

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

pub async fn creat_new_file(filename: &str)->Result<File, ErrorCode>
{
    let path = PATH.as_str();
    let full_path =  path.to_owned() +"/" + filename;

    let file = File::create_new(full_path).await
        .map_err(|_| ErrorExists)?;

    Ok(file)


}

pub async fn open_file(filename: &str) ->Result<File, ErrorCode>
{
    let path = PATH.as_str();
    let full_path = path.to_owned() + "/" + filename;

    let file = File::open(full_path).await
        .map_err(|_| ErrorCode::ErrorNotFound)?;
    Ok(file)
}



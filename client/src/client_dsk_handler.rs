use tokio::fs::File;
use protocol::utils::ErrorCode;
use protocol::utils::ErrorCode::ErrorExists;
const PATH : &str = r"C:\Users\shimo\Desktop\stuff";
pub async fn creat_new_file(filename: &str) ->Result<File, ErrorCode>
{
    let path = PATH;
    let full_path =  path.to_owned() +"/" + filename;

    let file = File::create_new(full_path).await
        .map_err(|_| ErrorExists)?;

    Ok(file)




}

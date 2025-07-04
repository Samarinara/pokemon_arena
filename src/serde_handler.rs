use tokio::fs::File;
use tokio::io::{AsyncReadExt, Error};


pub async fn load_json(path: &str) -> Result<String, Error>{
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("loaded");
    return Ok(contents);
}
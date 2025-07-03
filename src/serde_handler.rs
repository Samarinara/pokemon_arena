use tokio::fs::File;
use tokio::io::AsyncReadExt;


pub async fn load_json(path: &str) -> String{
    let mut file = File::open(path).await.unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.unwrap();
    println!("loaded");
    return contents;
}
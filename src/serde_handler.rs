use std::{
    fs::File, 
    io::Read
};


pub fn load_json(path: &str) -> String{
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);    
    println!("loaded");
    return contents;
}
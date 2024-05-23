use std::fs;
use std::path::Path;

pub fn save_file(file_name: &str, content: &[u8]) -> std::io::Result<()> {
    let path = Path::new("uploads").join(file_name);
    fs::create_dir_all(path.parent().unwrap())?; 
    fs::write(path, content) 
}
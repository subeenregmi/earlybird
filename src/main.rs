use std::{
    fs::{self, File, create_dir},
    io::Read,
    path::Path,
};

const DATA_DIR: &str = "/home/subeen/.local/share/earlybird/";
const DATA_FILE: &str = "data.csv";

fn main() {
    let dir_path = Path::new(DATA_DIR);

    let dir_exists = match dir_path.try_exists() {
        Ok(b) => b,
        Err(e) => panic!("error trying to check if dir path exists, {}", e),
    };

    if !dir_exists {
        if let Err(e) = fs::create_dir(dir_path) {
            panic!("error trying to create new dir, {}", e)
        }
    }
    let mut file: File;

    let mut file_path_str = String::from(DATA_DIR);
    file_path_str.push_str(DATA_FILE);

    let file_path = Path::new(&file_path_str);

    let file_exists = match file_path.try_exists() {
        Ok(b) => b,
        Err(e) => panic!("error trying to check if file exists {}", e),
    };

    if file_exists {
        println!("file exists!");
        file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("error trying to open file, {}", e),
        }
    } else {
        println!("file has just been created!");
        file = match File::create_new(file_path) {
            Ok(f) => f,
            Err(e) => panic!("error trying to create new file, {}", e),
        }
    }

    let mut buf = String::new();

    if let Err(e) = file.read_to_string(&mut buf) {
        panic!("error trying to read file, {}", e)
    }

    println!("File contents: {}", buf)
}

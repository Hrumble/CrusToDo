use std::fs::{self, File};
use crate::ListManager;
use std::{process::exit, env};
use std::path::PathBuf;
use toml;
use std::io::Write;

pub fn write_to_file(path : &PathBuf, listmanager : &ListManager){
    let toml_string : String = toml::to_string(listmanager).expect("Error when calling to_string on list manager");
    if fs::exists(path).unwrap() {
        let mut file : File = fs::File::open(path).expect("Error when opening file");
        let _ = file.write_all(toml_string.as_bytes());
    } else {
        let mut file : File = fs::File::create(path).expect("Error when opening file");
        let _ = file.write_all(toml_string.as_bytes());
    }
}

pub fn read_from_file(path : &PathBuf) -> ListManager{
   match fs::read_to_string(path) {
       Ok(val) => toml::from_str(&val).unwrap(),
       Err(_) => ListManager::new(),
   }

}

pub fn get_storage_path() -> PathBuf {
    let crustodo_path : &str = "crustodo";
    if cfg!(target_os = "windows"){
        match env::var("APPDATA") {
            Ok(var) => {
                let mut list_path : PathBuf = PathBuf::from(var);
                list_path.push(crustodo_path);
                list_path                
            },
            Err(_) => exit(1)
        }
    } else if cfg!(target_os = "macos") {
        let home = env::var("HOME").unwrap_or_else(|_| "/Users/default".to_string());
        let mut list_path : PathBuf = PathBuf::from(home);
        list_path.push(crustodo_path);
        list_path
    } else {
        // Linux-specific location
        let home = env::var("HOME").unwrap_or_else(|_| "/home/default".to_string());
        let mut list_path : PathBuf = PathBuf::from(home);
        list_path.push(crustodo_path);
        list_path
    }
}

pub fn check_or_create_file(path : &PathBuf) {
    let file_name : &str = "crustodo.toml";
    let mut new_path : PathBuf = path.clone();
    if !fs::exists(&new_path).expect("Could not check existence of list storage") {
        fs::create_dir(&new_path).expect("Failed to create necessary directories");
        println!("Created crustodo directory");
    }

    new_path.push(file_name);
    if !fs::exists(&new_path).expect("Failed to check existence of necessary files") {
        fs::File::create(new_path).expect("Failed to create necessary files");
        println!("Created crustodo.toml");
    }
}

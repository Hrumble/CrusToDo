use std::fs::{self, File};
use serde::{Deserialize, Serialize};
use std::option;
use crate::ListManager;
use std::{process::exit, env};
use std::path::PathBuf;
use std::io::Write;

pub fn write_to_file(crustodo_file : &mut File, listmanager : &ListManager){
    let json_string : String = serde_json::to_string_pretty(listmanager).expect("Error when calling to_string on list manager");
    // clears the file and write to it
    let _ = crustodo_file.set_len(0);
    let _ = crustodo_file.write_all(json_string.as_bytes());
}

pub fn read_from_file(path : &PathBuf) -> ListManager{
    let mut crustodo_path = path.clone(); 
    crustodo_path.push("crustodo.json");
    match fs::read_to_string(crustodo_path) {
        Ok(val) => serde_json::from_str(&val).unwrap_or_else(|_| {
            ListManager::new()
        }),
        Err(e) => {
            println!("{e}");
            ListManager::new()
        },
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

pub fn check_or_create_file(path : &PathBuf) -> File {
    let file_name : &str = "crustodo.json";
    let mut new_path : PathBuf = path.clone();
    if !fs::exists(&new_path).expect("Could not check existence of list storage") {
        fs::create_dir(&new_path).expect("Failed to create necessary directories");
        println!("Created crustodo directory");
    }

    new_path.push(file_name);
    if !fs::exists(&new_path).expect("Could not check existence of file") {
        let crust_file : File = fs::File::create(new_path).expect("Failed to create necessary files");
        println!("Created crustodo.json");
        crust_file
    } else {
       let crust_file : File = File::options().read(true).write(true).open(&new_path).expect("Failed opening crustodo file");
       crust_file
    }
    
}

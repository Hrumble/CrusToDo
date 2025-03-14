mod todolist;
mod task;
mod filehandler;
mod listmanager;
mod arghandler;

use std::path::PathBuf;
use std::fs::File;

use listmanager::ListManager;

fn main() {
    let storage_path : PathBuf = filehandler::get_storage_path();
    let mut listmanager : ListManager = filehandler::read_from_file(&storage_path);
    let mut crustodo_file : File = filehandler::check_or_create_file(&storage_path);
    // Parse the toml and store it in a ListManager

    match arghandler::handle_args(&mut listmanager) {
        Ok(_) => (),
        Err(e) => eprintln!("{e}"),
    }
    filehandler::write_to_file(&mut crustodo_file, &listmanager);
}







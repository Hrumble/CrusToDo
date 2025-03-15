mod todolist;
mod task;
mod filehandler;
mod listmanager;

use std::path::PathBuf;
use std::{env, process::exit};

use listmanager::ListManager;

fn main() {
    let storage_path : PathBuf = filehandler::get_storage_path();
    println!("{storage_path:?}");
    filehandler::check_or_create_file(&storage_path);
    // Parse the toml and store it in a ListManager
    let mut listmanager : ListManager = filehandler::read_from_file(&storage_path);

    handle_args(&mut listmanager);
    filehandler::write_to_file(&storage_path, &listmanager);
}

fn parse_args() -> Vec<String> {
    let args : Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help(HelpScreens::MAIN);
        exit(1)
    }
    args
}

fn handle_args(listmanager : &mut ListManager) {
    let args : Vec<String> = parse_args();
    if &args[1] == "list" {
        listmanager.print_lists();
    }
}

#[derive(PartialEq)]
enum HelpScreens {
    MAIN,
}

fn print_help(screen : HelpScreens){
    if screen == HelpScreens::MAIN {
        println!("
            Usage:

            crustodo [TODOLIST_NAME|list|create] [list|set|add] <TASK_ID|TASK_NAME>

            list - lists all tasks in the todo list, or lists all todo lists
            set - sets the status (completed/uncomplete) of the task <TASK_ID>
            add - Adds a new task to the todo list
        ");
    }
}

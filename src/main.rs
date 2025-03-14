mod todolist;
mod task;
mod listmanager;

use todolist::TodoList;
use task::Task;
use std::env::VarError;
use std::error::Error;
use std::fs;

use std::path::PathBuf;
use std::{env, process::exit};

fn main() {
    check_for_storage_path();
    parse_args();
}

fn parse_args() -> Vec<String> {
    let args : Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help(HelpScreens::MAIN);
        exit(1)
    }
    args
}

fn handle_args() {
    let args : Vec<String> = parse_args();
    if &args[1] == "init" {
       println!() 
    }
}

// This one checks if the environment variable CRUSTODO_PATH exists
// if it doesn't then prompt user to create it, if another error, then exits program
fn check_for_storage_path() -> PathBuf {
     match env::var("CRUSTODO_PATH") {
         Ok(val) => PathBuf::from(val),
         Err(VarError::NotPresent) => {
             println!("
                 Your crustodo list path has not been set yet, this might be because
                 this is the first time you've used crustodo, let's set it!

                 Enter the path where you want to store your lists:
             ");
             let mut user_input : String = String::new();
             std::io::stdin().read_line(&mut user_input).expect("Failed to read line");
             return PathBuf::from(user_input);
        }
        Err(e) => {
            eprintln!("There was an error reading the environment variable CRUSTODO_PATH : {e}");
            exit(1);
        }             
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

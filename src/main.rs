mod todolist;
mod task;
mod listmanager;

use todolist::TodoList;
use task::Task;

use std::{env, process::exit};

fn main() {
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
    if &args[1] == "create" {

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

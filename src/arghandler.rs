use crate::task::Task;
use crate::ListManager;
use std::process::exit;
use std::env;

fn parse_args() -> Vec<String> {
    let args : Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help(HelpScreens::Main);
        exit(1)
    }
    args
}

pub fn handle_args(listmanager : &mut ListManager) {
    let args : Vec<String> = parse_args();
    if &args[1] == "list" {
        listmanager.print_lists();
    } else if &args[1] == "create"{
        match args.get(2) {
            Some(val) => {
                listmanager.create_list(val).expect("Something went wrong");
            }
            None => {
               print_help(HelpScreens::CreateList); 
            }
        }
    } else {
        let list : Option<Task> = listmanager.lists.get(&args[1]);
        match list {
            Some(list) => {
                list.print_list();
            },
            None => {
                println!("No list with this name has been found");
            }
        }
    } 
}

#[derive(PartialEq)]
enum HelpScreens {
    Main,
    CreateList,
}

fn print_help(screen : HelpScreens){
    if screen == HelpScreens::Main {
        println!("
            Usage:

            crustodo [TODOLIST_NAME|list|create] [list|set|add] <TASK_ID|TASK_NAME>

            list - lists all tasks in the todo list, or lists all todo lists
            set - sets the status (completed/uncomplete) of the task <TASK_ID>
            add - Adds a new task to the todo list
        ");
    }
    exit(1);
}

use crate::todolist::TodoList;
use crate::{exit, ListManager};
use std::process::exit;
use std::env;

fn parse_args() -> Vec<String> {
    let args : Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help(HelpScreens::Main);
        
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
        if !listmanager.lists.contains_key(&args[1]){
            println!("This Crustodolist does not exist");
            exit(1);
        }
        if &args[2] == "add" {
           listmanager.create_task_ui(&args[1]); 
        } else if &args[2] == "remove" {
            let mut todo_list : &mut TodoList = listmanager.lists.get_mut(&args[1]).unwrap(); 
            let task_id : u16 = match args[3].trim().parse() {
                Ok(val) => val,
                Err(e) => {
                    println!("{e}");
                    
                }
            };
            todo_list.remove_task(&task_id).unwrap();
        }
        
    } 
}

#[derive(PartialEq)]
enum HelpScreens {
    Main,
    CreateList,
    RemoveTask,
}

fn print_help(screen : HelpScreens){
    if screen == HelpScreens::Main {
        println!("
            Usage:

            crustodo [TODOLIST_NAME|list|create] [set|add] <TASK_ID|TASK_NAME>

            list - lists all tasks in the todo list, or lists all todo lists
            set - sets the status (completed/uncomplete) of the task <TASK_ID>
            add - Adds a new task to the todo list
        ");
    }
    exit(1);
}

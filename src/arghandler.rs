use crate::todolist::TodoList;
use crate::ListManager;
use std::env;

fn parse_args() -> Vec<String> {
    let args : Vec<String> = env::args().collect();
    args
}

pub fn handle_args(listmanager: &mut ListManager) -> Result<(), String> {
    let args: Vec<String> = parse_args();
    
    // Ensure we have at least 2 arguments
    if args.len() < 2 {
        listmanager.print_lists();
        return Err("Invalid Input".to_string());
    }
    
    // Handle the "list" command
    if &args[1] == "list" {
        listmanager.print_lists();
        return Ok(()); // Return Ok(())
    } 
    // Handle the "create" command
    else if &args[1] == "create" {
        match args.get(2) {
            Some(val) => {
                listmanager.create_list(val)?;
                return Ok(()); // Return Ok(())
            }
            None => {
                return print_help(HelpScreens::CreateList); // Properly propagate Result
            }
        }
    } 
    // Handle the "help" command
    else if &args[1] == "help" {
        return print_help(HelpScreens::Main); // Properly propagate Result
    } 
    else { 
        // Ensure the list exists before proceeding
        if !listmanager.lists.contains_key(&args[1]) {
            println!("This Crustodolist does not exist");
            return Err("Tried to reference an unexisting list".to_string());
        }
        
        // Safely unwrap the TodoList by getting a mutable reference
        let todo_list: &mut TodoList = listmanager.lists.get_mut(&args[1]).unwrap();
        
        match args.get(2) {
            Some(_) => (),
            None => {
                todo_list.print_list();
                return Ok(()); // Return Ok(())
            },
        }
        
        // Handle the "add" command
        if &args[2] == "add" {
            return listmanager.create_task_ui(&args[1]);
        } 
        // Handle the "remove" command
        else if &args[2] == "remove" {
            let task_id: u16 = match args[3].trim().parse() {
                Ok(val) => val,
                Err(_) => {
                    return Err("You need to enter a valid task ID".to_string());
                }
            };
            return todo_list.remove_task(&task_id); // Return the result of remove_task
        } 
        // Handle the "set" command
        else if &args[2] == "set" {
            if args.len() < 5 {
                return print_help(HelpScreens::SetTask); // Properly propagate Result
            }
            
            // Parse task ID
            let task_id: u16 = match args[3].trim().parse() {
                Ok(val) => val,
                Err(_) => {
                    return Err("You need to enter a valid task ID".to_string());
                }
            };

            // Parse completion status
            let completed: bool = match args[4].trim() {
                "complete" => true,
                "todo" => false,
                _ => {
                    return Err("Invalid completion status. Use 'complete' or 'todo'.".to_string());
                }
            };
            
            // Mark task as completed (or todo)
            todo_list.mark_task_completed(&task_id, completed)?;
            todo_list.print_task(&task_id)?;
            return Ok(()); // Return Ok(()) after completing the task
        }
    }
    print_help(HelpScreens::Main)
}

#[derive(PartialEq)]
enum HelpScreens {
    Main,
    CreateList,
    RemoveTask,
    SetTask,
}

fn print_help(screen : HelpScreens) -> Result<(), String>{
    if screen == HelpScreens::Main {
        println!("
Usage:

crustodo [TODOLIST_NAME|create] [set|add] <TASK_ID>

list - lists all tasks in the todo list, or lists all todo lists
set - sets the status (completed/uncomplete) of the task <TASK_ID>
add - Adds a new task to the todo list

Examples:
$ crustodo create groceries ; Creates a new crustodo list called groceries

$ crustodo groceries ; displays all tasks inside the groceries list

$ crustodo groceries add ; prompts the program to create a new task

$ crustodo groceries set 2 complete ; marks the task with id=2 to complete

$ crustodo groceries remove 2 ; removes the task with id=2
        ");
    } else if screen == HelpScreens::CreateList {
        println!("
To create a todo list use:
$ crustodo create list_name
            ");
    } else if screen == HelpScreens::RemoveTask {
        println!("
To remove a task, use:
$ crustodo <list_name> remove <task_id>

Where <list_name> is the name of the crustodo list where the task is located,
and <task_id> is the id of the task to be removed.

You can use:
$ crustodo <list_name>
to get a list of all tasks and their id in a particular crustodo list.
        ");
    } else if screen == HelpScreens::SetTask {
        println!("
to set a task, use:
$ crustodo <list_name> set complete|todo <task_id>

Where <list_name> is the name of the crustodo list where the task is located,
and <task_id> is the id of the task to be set.

complete - sets the task to completed ✅
todo - sets the task to pending/todo ❌

You can use:
$ crustodo <list_name>
to get a list of all tasks and their id in a particular crustodo list.

            ");
    }
    Err("Error : Invalid input".to_string())
}

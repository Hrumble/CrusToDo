use crate::task::Task;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/* 
A todolist is a container for Tasks, so you can have multiple todo lists each with their own
tasks, reference each task via their ID, so HashMap id(u16) -> task (Task)
*/
#[derive(Debug, Deserialize, Serialize)]
pub struct TodoList {
    pub name : String,
    tasks : HashMap<u16, Task>,
    id : u16,
}

impl TodoList {
    pub fn new(list_name : &str) -> Self {
       Self {
           name : list_name.to_string(),
           tasks : HashMap::new(),
           id : 1,
       } 
    }
    // Simple function to check if task exists
    fn task_exists(&mut self, task_id : &u16) -> bool {
        self.tasks.contains_key(task_id)
    }
    
    // Attemps to create a task with a particular ID, throws an error if the ID already exists
    // inside the todolist
    pub fn create_task(&mut self, task_name : &str, task_definition : &str, task_id : &u16) -> Result<&Task, String> {
        // Creates task
        let new_task = Task {
            name : task_name.to_string(),
            description : task_definition.to_string(),
            completed : false,
           };
        // If task exists, return error, otherwise insert the new task
        if self.task_exists(task_id) {
            Err(format!("A task with id {} already exists, failed to add", task_id))
        } else {
            self.tasks.insert(*task_id, new_task);
            match self.tasks.get(task_id) {
                Some(task) => Ok(task),
                None => Err(String::from("An error occured when trying to add task")),
            }  
        }
    }
    // Removes a task, if it fails, return error
    pub fn remove_task(&mut self, task_id : &u16) -> Result<(), String>{
        match self.tasks.remove(task_id) {
            Some(_) => Ok(()),
            None => Err(format!("Could not remove task with id {}, are you sure it exists?", task_id))
        }
    }

    // You can guess what this one does 
    pub fn mark_task_completed(&mut self, task_id : &u16, completed : bool) -> Result<(), String> {
        match self.tasks.get_mut(task_id) {
            Some(task) => Ok(task.mark_completed(completed)),
            None => Err(format!("Task with id {} does not exist. Are you sure this is the correct todo list?", task_id)),
        }
        
    }

    //UI UX Functions
    pub fn print_task(&self, task_id : &u16) -> Result<(), String> {
        match self.tasks.get(task_id) {
            Some(task) => Ok(task.print()),
            None => Err(format!("Task with id {} does not exist. Are you sure this is the correct todo list?", task_id)),
        }
    }

    pub fn print_list(&self) {
        for (id, task) in &self.tasks {
            println!("==============Task ID {}============", id);
            task.print();
        }
    }
}

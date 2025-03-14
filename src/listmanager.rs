use std::{collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};
use crate::{task::Task, todolist::TodoList};

#[derive(Debug, Deserialize, Serialize)]
pub struct ListManager {
    pub lists : HashMap<String, TodoList>,
    pub current_id : u16,
}

impl ListManager {
    pub fn new() -> Self{
        Self {
            lists : HashMap::new(),
            current_id : 1,
        }
    }
    pub fn print_lists(&self){
        if self.lists.len() < 1{
            println!("
You have no Crutodolists📃 created, to create one, use:
$ crustodo create list_name
");
            return;
        }

        for (_, list) in &self.lists {
            println!("📃 {} - {} tasks", list.name, list.tasks.len());
        }
    } 
    // creates a list, todo list 
    pub fn create_list(&mut self, list_name : &str) -> Result<&TodoList, String> {
        if self.lists.contains_key(list_name) {
            return Err(format!("Crustodo list with name {} already exists", list_name));
        }
        
        self.lists.insert(list_name.to_string(), TodoList::new(list_name));
        match self.lists.get(list_name){
            Some(list) => {
                println!("Successfully created {list_name}📃");
                Ok(list)
            },
            None => Err(String::from("There was an error creating the list manager"))
        } 
    }

    pub fn remove_list(&mut self, list_name : &str) -> Result<(), String> {
        match self.lists.remove(list_name) {
             Some(_a) => {
                println!("Successfully removed 📃{list_name}");
                Ok(())
             },
             None => Err("This list does not exist, did you mispell it?".to_string()),
         } 
        
    }

    pub fn create_task(&mut self, list_name : &str, task_name : &str, task_description : &str) -> Result<&Task, String> {
        match self.lists.get_mut(list_name) {
            Some(todo_list) => {
                let task : &Task = todo_list.create_task(task_name, task_description, self.current_id).unwrap();
                self.current_id += 1;
                Ok(task)
            },
            None => {
                Err(format!("Crustodo list with name {} does not exist", list_name))
            }
        }
    }

    pub fn create_task_ui(&mut self, list_name : &str) -> Result<(), String>{
        println!("Task Name: ");
        let mut task_name : String = String::new();
        std::io::stdin().read_line(&mut task_name);
        println!("Task Description: ");
        let mut task_description : String = String::new();
        std::io::stdin().read_line(&mut task_description);

        match self.create_task(list_name, &task_name, &task_description) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    pub fn remove_task(&mut self, list_name : &str, task_id : &u16) {
        match self.lists.get_mut(list_name) {
            Some(todo_list) => {
                todo_list.remove_task(task_id).unwrap();
            },
            None => {}
        }
    }
}

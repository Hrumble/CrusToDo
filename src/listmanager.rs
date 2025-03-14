use std::collections::HashMap;

use crate::todolist::TodoList;

pub struct ListManager {
    pub lists : HashMap<String, TodoList>
}

impl ListManager {
    pub fn print_lists(&self){
        for (_, list) in &self.lists {
            println!("📃 {}", list.name);
        }
    } 
    
    pub fn create_list(&mut self, list_name : &str) -> Result<&TodoList, String> {
        if self.lists.contains_key(list_name) {
            return Err(format!("Crustodo list with name {} already exists", list_name));
        }

        self.lists.insert(list_name.to_string(), TodoList::new(list_name));
        match self.lists.get(list_name){
            Some(list) => Ok(list),
            None => Err(String::from("There was an error creating the list manager"))
        } 
    }

    pub fn remove_list(&mut self, list_name : &str) -> Result<(), String> {
        match self.lists.remove(list_name) {
            Some(_) => Ok(()),
            None => Err(format!("could not remove {}, are you sure it exists?", list_name)),
        }
    }
}

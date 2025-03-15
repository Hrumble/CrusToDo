use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub name : String,
    pub description : String,
    pub completed : bool,
}

impl Task {
    pub fn mark_completed(&mut self, completed : bool) {
        self.completed = completed;
    }

    pub fn print(&self){
        println!(
            "
Name: {}
Description: {}
Status: {}
            ", 
            self.name,
            self.description,
            if self.completed {"✅ Completed"} else {"❌ Awaiting Completion"},
        )
    }
}

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
            {}\n
            Description: {}\n
            Status: {}
            ", 
            self.name,
            self.description,
            if self.completed {"✅"} else {"❌"},

        )
    }
}

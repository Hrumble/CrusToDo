mod todolist;
mod task;

use todolist::TodoList;
use task::Task;

fn main() {
    let mut list : TodoList = TodoList::new(String::from("My Todo List")); 
    
    let new_task : &Task = list.create_task("My task", "Eat someone's ass", &1).unwrap();
    let other_task : &Task = list.create_task("My task", "Eat someone's ass", &1).unwrap();

}

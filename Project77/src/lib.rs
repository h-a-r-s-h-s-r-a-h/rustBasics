pub mod list {
    pub struct Tasks {
        pub item: String,
    }
}

mod things_todo;
use crate::things_todo::add_activity;
use things_todo::items_completed;

fn lets_add(){
    let task = list::Tasks{item: String::from("Do the dishes")};
    add_activity(); 
    items_completed::remove_task();
}
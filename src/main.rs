use std::env;

struct TodoItem {
    name: String,
    completed: char
}

impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem{
            name: name,
            completed: ' '
        };
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();
    let todo_item_1 = TodoItem::new("Say hi to CJ".to_string());
    let todo_item_2 = TodoItem::new("Do something with Rust".to_string());
    let todo_list = vec![todo_item_1, todo_item_2];

    if command == "get" {
        for item in todo_list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }
}

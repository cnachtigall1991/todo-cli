use std::env;

struct TodoItem {
    name: String,
    completed: char
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();
    let todo_item = TodoItem{
        name: "Say hi to CJ",
        completed: ' '
    };
    

    if command == "get" {
        
    }
}

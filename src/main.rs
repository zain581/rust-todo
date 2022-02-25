
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: u64,
    task: String,
}

impl Todo {
    fn new(id: u64, task: &str) -> Todo {
        Todo {
            id,
            task: task.to_string(),
        }
    }
}
use std::fs;

struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { todos: Vec::new() }
    }

    fn add(&mut self, task: &str) {
        let id = self.todos.len() as u64 + 1;
        let todo = Todo::new(id, task);
        self.todos.push(todo);
    }

    fn list(&self) {
        for todo in &self.todos {
            println!("{}: {}", todo.id, todo.task);
        }
    }

    fn remove(&mut self, id: u64) {
        self.todos.retain(|todo| todo.id != id);
    }

    fn save_to_file(&self, file_path: &str) {
        let json = serde_json::to_string_pretty(&self.todos).unwrap();
        fs::write(file_path, json).expect("Unable to write to file");
    }

    fn load_from_file(file_path: &str) -> TodoList {
        if let Ok(contents) = fs::read_to_string(file_path) {
            if let Ok(todos) = serde_json::from_str(&contents) {
                return TodoList { todos };
            }
        }
        TodoList::new()
    }
}

use std::env;

fn main() {
    let file_path = "todos.json";
    let mut todo_list = TodoList::load_from_file(file_path);

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [add|list|remove] [task]", args[0]);
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: {} add [task]", args[0]);
                return;
            }
            let task = &args[2..].join(" ");
            todo_list.add(task);
            todo_list.save_to_file(file_path);
        }
        "list" => todo_list.list(),
        "remove" => {
            if args.len() < 3 {
                println!("Usage: {} remove [id]", args[0]);
                return;
            }
            if let Ok(id) = args[2].parse::<u64>() {
                todo_list.remove(id);
                todo_list.save_to_file(file_path);
            } else {
                println!("Invalid ID");
            }
        }
        _ => {
            println!("Unknown command");
        }
    }
}

use std::f64;

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// penny = 1
// Nickel = 5
// dime = 10
// Quarter = 25

fn coin_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum Menu {
    Create,
    Update,
    MarkComplete,
    Delete,
    Get,
    Exit,
}

#[derive(Debug, Clone, PartialEq)]
struct Todo {
    id: u8,
    title: String,
    is_completed: Status,
}

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Incomplete,
    Completed,
    Pending,
}

struct TodoList {
    data: Vec<Todo>,
    next_id: u8,
}

impl TodoList {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            next_id: 1,
        }
    }

    fn create_todo(&mut self, title: String) -> u8 {
        let current_id = self.next_id;

        let todo = Todo {
            id: current_id,
            title,
            is_completed: Status::Incomplete,
        };

        self.next_id += 1;
        self.data.push(todo);
        current_id
    }

    fn mark_todo(&mut self, id: u8) -> bool {
        if let Some(todo_item) = self.data.iter_mut().find(|todo_id| todo_id.id == id) {
            todo_item.is_completed = Status::Completed;
            true
        } else {
            false
        }
    }
    fn delete_todo2(&mut self, id: u8) -> bool {
        if let Some(user_todo_index) = self.data.iter_mut().position(|todo_id| todo_id.id == id) {
            self.data.remove(user_todo_index);
            true
        } else {
            false
        }
    }

    fn delete_todo(&mut self, id: u8) {
        self.data.retain(|todo_id| todo_id.id != id)
    }

    fn update_todo(&mut self, id: u8, title: String) -> bool {
        for todo in self.data.iter_mut() {
            if todo.id == id {
                todo.title = title;
                return true;
            }
        }

        false
    }

    fn update_todo_if_let(&mut self, id: u8, new_title: String) -> bool {
        if let Some(user_todo) = self.data.iter_mut().find(|todo_id| todo_id.id == id) {
            user_todo.title = new_title;
            true
        } else {
            false
        }
    }

    fn update_todo_method_qubzes(&mut self, id: u8, title: String) -> String {
        let data = self
            .data
            .iter_mut()
            .find(|todo| todo.id == id)
            .expect("failed");
        data.title = title.clone();
        title
    }

    fn get_single_todo(&self, id: u8) -> &Todo {
        self.data.iter().find(|todo| todo.id == id).expect("failed")
    }

    fn get_all_todo(&self) -> &Vec<Todo> {
        &self.data
    }
}

fn main() {
    let num = 10;
    let decimal = num as f64;
    println!("Value: {decimal:.2}")
}

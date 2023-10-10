use std::{
    io::{self, Write},
    rc::Rc,
};

pub struct Todo {
    description: Rc<str>,
    completed: bool,
}

impl Todo {
    pub fn new(description: Rc<str>) -> Self {
        Self {
            description,
            completed: false,
        }
    }

    pub fn toggle_complete(&mut self) {
        self.completed = !self.completed
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn edit(&mut self) {
        let colour = if self.completed() { GREEN } else { RED };

        println!(
            "{colour}{}\x1b[0m\nNew Description:{colour}",
            self.description()
        );

        let mut new_description = String::new();
        io::stdin()
            .read_line(&mut new_description)
            .expect("read line");

        // Reset colour
        print!("\x1b[0m");

        self.description = new_description.trim_end().into();
    }
}

pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
const BOLD: &str = "\x1b[1m";

pub trait TodoList {
    fn view_tasks(&self, selected_index: usize);
    fn add_todo(&mut self);
}

impl TodoList for Vec<Todo> {
    fn add_todo(&mut self) {
        let todos_string = self
            .iter()
            .map(|todo| {
                let colour = if todo.completed() { GREEN } else { RED };

                format!("{colour}{}\x1b[0m", todo.description())
            })
            .collect::<Vec<_>>()
            .join("\n");

        println!("Input Description:\n{todos_string}{RED}");

        io::stdout().flush().expect("flush stdout");

        let mut description = String::new();
        io::stdin().read_line(&mut description).expect("read line");

        self.push(Todo::new(description.trim_end().into()));
    }

    fn view_tasks(&self, selected_index: usize) {
        let todos_to_string = move |(i, todo): (usize, &Todo)| {
            let colour = if todo.completed() { GREEN } else { RED };
            let highlight = if i == selected_index { BOLD } else { "\x1b[0m" };

            format!("{highlight}{colour}{}\x1b[0m", todo.description())
        };

        let todos_string = self
            .iter()
            .enumerate()
            .map(todos_to_string)
            .collect::<Vec<_>>()
            .join("\n");

        println!("{}", todos_string);
    }
}

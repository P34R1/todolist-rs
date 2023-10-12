use std::{io, rc::Rc};

use crate::{GREEN, RED};

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

        self.description = new_description.trim_end().into();
    }
}

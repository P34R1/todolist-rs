use crate::{todo::Todo, BOLD, GREEN, RED};
use std::io::{stdin, stdout, Write};

pub fn add_todo(todos: &mut Vec<Todo>) {
    let todos_string = todos
        .iter()
        .map(|todo| {
            let colour = if todo.completed() { GREEN } else { RED };

            [colour, todo.description()].concat()
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("Input Description:\n{todos_string}{RED}");

    Write::flush(&mut stdout()).expect("flush stdout");

    let mut description = String::new();
    stdin().read_line(&mut description).expect("read line");

    todos.push(Todo::new(description.trim_end().into()));
}

pub fn view_tasks(todos: &[Todo], selected_index: usize) {
    let todos_to_string = move |(i, todo): (usize, &Todo)| {
        let colour = if todo.completed() { GREEN } else { RED };
        let highlight = if i == selected_index { BOLD } else { "\x1b[0m" };

        [highlight, colour, todo.description()].concat()
    };

    let todos_string = todos
        .iter()
        .enumerate()
        .map(todos_to_string)
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", todos_string);
}

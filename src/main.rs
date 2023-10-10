use todolist_rs::{Todo, TodoList};

extern "C" {
    fn _getch() -> u8;
}

fn cls() {
    print!("\x1b[0m\x1b[H\x1b[J");
}

const INSTRUCTIONS: &str = "'A'dd 'D'elete 'E'dit 'T'oggle complete 'Q'uit. Up Arrow Down Arrow.";

const ADD: char = 'a';
const DELETE: char = 'd';
const EDIT: char = 'e';
const TOGGLE: char = 't';
const QUIT: char = 'q';

const UP_ARROW: char = 'H';
const DOWN_ARROW: char = 'P';

fn main() {
    let mut todos = vec![
        Todo::new("Learn Rust".into()),
        Todo::new("Learn C".into()),
        Todo::new("Learn Zig".into()),
    ];

    let mut selected_index = 0;

    loop {
        cls();

        println!("{}", INSTRUCTIONS);
        todos.view_tasks(selected_index);

        let ch = unsafe { _getch() } as char;

        cls();

        if todos.is_empty() && !matches!(ch, ADD | QUIT) {
            continue;
        }

        match ch {
            QUIT => return,

            UP_ARROW => selected_index = selected_index.saturating_sub(1),
            DOWN_ARROW => selected_index = (selected_index + 1).clamp(0, todos.len() - 1),

            TOGGLE => todos[selected_index].toggle_complete(),

            EDIT => todos[selected_index].edit(),

            ADD => todos.add_todo(),
            DELETE => {
                todos.remove(selected_index);

                if selected_index == todos.len() {
                    selected_index = selected_index.saturating_sub(1);
                }
            }

            _ => {}
        }
    }
}

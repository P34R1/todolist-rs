mod todo;
mod todolist;

use todo::Todo;
use todolist::{add_todo, view_tasks};

extern "C" {
    fn _getch() -> u8;
}

fn cls() {
    print!("\x1b[H\x1b[J");
}

// ANSI Colours
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const BOLD: &str = "\x1b[1m";

const INSTRUCTIONS: &str =
    "\x1b[0m'A'dd 'D'elete 'E'dit 'T'oggle complete 'Q'uit. Up Arrow Down Arrow.";

// Keybinds
const ADD: char = 'a';
const DELETE: char = 'd';
const EDIT: char = 'e';
const TOGGLE: char = 't';
const QUIT: char = 'q';
const UP_ARROW: char = 'H';
const DOWN_ARROW: char = 'P';

fn main() {
    let mut todos = Vec::from([
        Todo::new("Learn Rust".into()),
        Todo::new("Learn C".into()),
        Todo::new("Learn Zig".into()),
    ]);

    let mut selected_index: usize = 0;

    loop {
        cls();

        println!("{}", INSTRUCTIONS);
        view_tasks(&todos, selected_index);

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

            ADD => add_todo(&mut todos),
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

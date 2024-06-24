// src/main.rs

mod display {
    pub mod terminal_display;
}
mod engine {
    pub mod material;
    pub mod board;
}

use engine::board::Board;

use crate::display::terminal_display::TerminalDisplay;

fn main() {
    let mut display = TerminalDisplay::new();

    let mut board = Board::new();

    while display.run(&mut board) {
        // Loop to process input until 'q' is pressed
    }
    print!("\n\rExiting...\n\r")
}


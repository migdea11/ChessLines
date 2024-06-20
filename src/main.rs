// src/main.rs

mod display {
    pub mod terminal_display;
}
mod engine {
    pub mod material;
}

use crate::display::terminal_display::TerminalDisplay;
use crate::engine::material::Material;

fn main() {
    let mut display = TerminalDisplay::new();

    let mut board = [
        [
            Material::Rook.get_black_code(),
            Material::Knight.get_black_code(),
            Material::Bishop.get_black_code(),
            Material::Queen.get_black_code(),
            Material::King.get_black_code(),
            Material::Bishop.get_black_code(),
            Material::Knight.get_black_code(),
            Material::Rook.get_black_code(),
        ],
        [
            Material::Pawn.get_black_code(); 8 // Using array repeat syntax for simplicity
        ],
        [' '; 8], // Empty row of spaces
        [' '; 8], // Empty row of spaces
        [' '; 8], // Empty row of spaces
        [' '; 8], // Empty row of spaces
        [
            Material::Pawn.get_white_code(); 8 // Using array repeat syntax for simplicity
        ],
        [
            Material::Rook.get_white_code(),
            Material::Knight.get_white_code(),
            Material::Bishop.get_white_code(),
            Material::Queen.get_white_code(),
            Material::King.get_white_code(),
            Material::Bishop.get_white_code(),
            Material::Knight.get_white_code(),
            Material::Rook.get_white_code(),
        ],
    ];

    display.display_board(&board);

    while display.process_input(&mut board) {
        // Loop to process input until 'q' is pressed
    }
}

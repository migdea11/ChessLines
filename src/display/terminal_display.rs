use std::io::{stdin, stdout, Stdout, Write};
use termion::input::TermRead;
use termion::{cursor, clear, color, style};
use termion::raw::{RawTerminal, IntoRawMode};
use crate::engine::board::Board;
use crate::engine::material::{Colour, Material, MaterialType};

pub struct TerminalDisplay {
    stdout: RawTerminal<Stdout>,
}

// This should be split into the ./src/app/app.rs for the "run" logic, only the IO should be in the terminal_display.rs
impl TerminalDisplay {
    pub fn new() -> TerminalDisplay {
        let stdout = stdout().into_raw_mode().unwrap();
        TerminalDisplay { stdout }
    }

    pub fn run(&mut self, board: &mut Board) -> bool {
        self.display_board(&board);

        print!("Enter command: ");
        self.stdout.flush().unwrap();

        let input = self.read_line();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let command = parts.get(0).copied();  // Use copied to directly copy the &str reference
        match command {
            Some("q") | Some("quit") => return false,
            Some("h") | Some("help") => self.show_help(),
            Some("s") | Some("select") => {
                if parts.len() < 2 {
                    println!("Error: No piece location specified. Usage: s <piece_location>");
                } else {
                    self.select_piece(parts[1], board);
                }
            },
            Some("m") | Some("move") => {
                if parts.len() < 3 {
                    println!("Error: Insufficient arguments for move. Usage: m <from_location> <to_location>");
                } else {
                    self.move_piece(parts[1], parts[2], board);
                }
            },
            _ => println!("Unknown command. Type 'h' or 'help' for help."),
        }
        true
    }

    fn get_code(material: &Material) -> char {
        return match material.colour {
            Colour::White => match material.material_type {
                MaterialType::King => '♔',   // White King
                MaterialType::Queen => '♕',  // White Queen
                MaterialType::Rook => '♖',   // White Rook
                MaterialType::Bishop => '♗', // White Bishop
                MaterialType::Knight => '♘', // White Knight
                MaterialType::Pawn => '♙',   // White Pawn
            }
            Colour::Black => match material.material_type {
                MaterialType::King => '♚',   // Black King
                MaterialType::Queen => '♛',  // Black Queen
                MaterialType::Rook => '♜',   // Black Rook
                MaterialType::Bishop => '♝', // Black Bishop
                MaterialType::Knight => '♞', // Black Knight
                MaterialType::Pawn => '♟',   // Black Pawn
            }
        }
    }

    fn display_board(&mut self, board: &Board) {
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap(); // Clear the screen and move cursor to top-left
        let mut grid_colour_toggle = true;
        let grid_light_colour = color::Bg(color::LightWhite);
        let grid_dark_colour = color::Bg(color::LightBlack);
        let material_colour = color::Fg(color::Black);
        for row in board.squares.iter() {
            for &material in row {
                let material_char = if material.is_some() { Self::get_code(&material.unwrap()) } else { ' ' };
                if grid_colour_toggle {
                    write!(self.stdout, "{}{}{} ", grid_light_colour, material_colour, material_char).unwrap();
                } else {
                    write!(self.stdout, "{}{}{} ", grid_dark_colour, material_colour, material_char).unwrap();
                };
                grid_colour_toggle = !grid_colour_toggle;
            }
            writeln!(self.stdout, "{}\r", style::Reset).unwrap(); // New line after each row
            grid_colour_toggle = !grid_colour_toggle; // Toggle the grid colour for each row
            //self.stdout.flush().unwrap(); // Flush stdout to apply all writes
        }
        // Move cursor to the bottom of the terminal and display a prompt
        let (_, height) = termion::terminal_size().unwrap(); // Get terminal dimensions
        write!(self.stdout, "{}> ", cursor::Goto(1, height)).unwrap(); // Move cursor to the start of the last line and display '>'
        self.stdout.flush().unwrap(); // Flush stdout to apply all writes
    }

    fn read_line(&mut self) -> String {
        let stdin = stdin();
        let mut input = String::new();

        // Iterate over keys pressed
        for c in stdin.keys() {
            let key = c.unwrap();
            match key {
                termion::event::Key::Char('\n') => {
                    // Break the loop if the Enter key is pressed
                    break;
                },
                termion::event::Key::Char(c) => {
                    // Append the character to our input string
                    input.push(c);
                    // Echo the character back to the terminal
                    self.stdout.write(c.to_string().as_bytes()).unwrap();
                    self.stdout.flush().unwrap();
                },
                termion::event::Key::Backspace => {
                    // Handle backspace: remove the last character from input
                    input.pop();
                    // Move the cursor back, print a space to erase, and then move back again
                    self.stdout.write(b"\x08 \x08").unwrap();
                    self.stdout.flush().unwrap();
                },
                _ => {}
            }
        }

        input
    }

    fn show_help(&self) {
        println!("Commands available:\r");
        println!("  q, quit: Exit the program\r");
        println!("  h, help: Show this help menu\r");
        println!("  s, select <piece_location>: Select a piece. Example -> s e2\r");
        println!("  m, move <piece_move>: Move a piece. Example -> m fxe2\n");
    }

    fn select_piece(&self, location: &str, _board: &mut Board) {
        println!("Selecting piece at {}", location);
        // Add logic to handle piece selection
    }

    fn move_piece(&self, from: &str, to: &str, _board: &mut Board) {
        println!("Moving piece from {} to {}", from, to);
        // Add logic to handle moving a piece
    }
}

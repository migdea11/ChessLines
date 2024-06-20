use std::io::{stdin, stdout, Stdout, Write};
use termion::{input::TermRead, event::Key, cursor, clear, color, style};
use termion::raw::{RawTerminal, IntoRawMode};

pub struct TerminalDisplay {
    stdout: RawTerminal<Stdout>,
}

impl TerminalDisplay {
    pub fn new() -> TerminalDisplay {
        let stdout = stdout().into_raw_mode().unwrap();
        TerminalDisplay { stdout }
    }

    pub fn display_board(&mut self, board: &[[char; 8]; 8]) {
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap(); // Clear the screen and move cursor to top-left
        let mut grid_colour_toggle = true;
        let grid_light_colour = color::Bg(color::LightWhite);
        let grid_dark_colour = color::Bg(color::LightBlack);
        let material_colour = color::Fg(color::Black);
        for row in board {
            for &piece in row {
                if grid_colour_toggle {
                    write!(self.stdout, "{}{}{} ", grid_light_colour, material_colour, piece).unwrap();
                } else {
                    write!(self.stdout, "{}{}{} ", grid_dark_colour, material_colour, piece).unwrap();
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

    pub fn update_board(_board: &mut [[char; 8]; 8]) {
        // Implement logic to modify the board
    }

    pub fn process_input(&mut self, board: &mut [[char; 8]; 8]) -> bool {
        let stdin = stdin();
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => return false,
                Key::Char(' ') => {
                    Self::update_board(board);
                    self.display_board(board);
                },
                _ => {}
            }
        }
        true
    }
}

// TODO this will eventually be the base class for all materials. The current content should be moved to terminal_display.rs

// material.rs

pub enum Material {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Material {
    pub fn get_white_code(&self) -> char {
        match self {
            Material::King => '♔',   // White King
            Material::Queen => '♕',  // White Queen
            Material::Rook => '♖',   // White Rook
            Material::Bishop => '♗', // White Bishop
            Material::Knight => '♘', // White Knight
            Material::Pawn => '♙',   // White Pawn
        }
    }

    pub fn get_black_code(&self) -> char {
        match self {
            Material::King => '♚',   // Black King
            Material::Queen => '♛',  // Black Queen
            Material::Rook => '♜',   // Black Rook
            Material::Bishop => '♝', // Black Bishop
            Material::Knight => '♞', // Black Knight
            Material::Pawn => '♟',   // Black Pawn (corrected to be just the pawn character)
        }
    }
}


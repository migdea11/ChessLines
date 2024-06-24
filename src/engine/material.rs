// TODO this will eventually be the base class for all materials. The current content should be moved to terminal_display.rs

// material.rs

// Assuming definitions in material.rs or similar
#[derive(Copy, Clone)]
pub enum MaterialType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone)]
pub enum Colour {
    White,
    Black,
}

#[derive(Copy, Clone)]
pub struct Material {
    pub material_type: MaterialType,
    pub colour: Colour,
}


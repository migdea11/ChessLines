use super::material::{Material, MaterialType, Colour};

pub struct Board {
    pub squares: [[Option<Material>; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            squares: [[None; 8]; 8],
        };

        // Setup pawns
        for i in 0..8 {
            board.squares[1][i] = Some(Material {
                material_type: MaterialType::Pawn,
                colour: Colour::Black,
            });
            board.squares[6][i] = Some(Material {
                material_type: MaterialType::Pawn,
                colour: Colour::White,
            });
        }

        // Setup rooks
        board.squares[0][0] = Some(Material {
            material_type: MaterialType::Rook,
            colour: Colour::Black,
        });
        board.squares[0][7] = Some(Material {
            material_type: MaterialType::Rook,
            colour: Colour::Black,
        });
        board.squares[7][0] = Some(Material {
            material_type: MaterialType::Rook,
            colour: Colour::White,
        });
        board.squares[7][7] = Some(Material {
            material_type: MaterialType::Rook,
            colour: Colour::White,
        });

        // Setup knights
        board.squares[0][1] = Some(Material {
            material_type: MaterialType::Knight,
            colour: Colour::Black,
        });
        board.squares[0][6] = Some(Material {
            material_type: MaterialType::Knight,
            colour: Colour::Black,
        });
        board.squares[7][1] = Some(Material {
            material_type: MaterialType::Knight,
            colour: Colour::White,
        });
        board.squares[7][6] = Some(Material {
            material_type: MaterialType::Knight,
            colour: Colour::White,
        });

        // Setup bishops
        board.squares[0][2] = Some(Material {
            material_type: MaterialType::Bishop,
            colour: Colour::Black,
        });
        board.squares[0][5] = Some(Material {
            material_type: MaterialType::Bishop,
            colour: Colour::Black,
        });
        board.squares[7][2] = Some(Material {
            material_type: MaterialType::Bishop,
            colour: Colour::White,
        });
        board.squares[7][5] = Some(Material {
            material_type: MaterialType::Bishop,
            colour: Colour::White,
        });

        // Setup queens
        board.squares[0][3] = Some(Material {
            material_type: MaterialType::Queen,
            colour: Colour::Black,
        });
        board.squares[7][3] = Some(Material {
            material_type: MaterialType::Queen,
            colour: Colour::White,
        });

        // Setup kings
        board.squares[0][4] = Some(Material {
            material_type: MaterialType::King,
            colour: Colour::Black,
        });
        board.squares[7][4] = Some(Material {
            material_type: MaterialType::King,
            colour: Colour::White,
        });

        board
    }
}

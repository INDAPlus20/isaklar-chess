use std::fmt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Color {
    Black,
    White,
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Piece {
    color: Color,
    title: PieceType,
}

impl Piece {
    pub fn new(color: Color, title: PieceType) -> Piece {
        Piece { color, title }
    }

    pub fn title(&self) -> &PieceType {
        &self.title
    }

    pub fn moves(&self, position: u32) -> Vec<u32> {
        vec![]
    }
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl PieceType {
    pub fn directions(&self) -> Vec<(i32, i32, bool)> {
        match &self {
            PieceType::King => vec![
                (-1, 1, false),
                (0, 1, false),
                (1, 0, false),
                (1, 1, false),
                (-1, 0, false),
                (-1, -1, false),
                (0, -1, false),
                (1, -1, false),
            ],
            PieceType::Queen => vec![
                (-1, 1, true),
                (0, 1, true),
                (1, 0, true),
                (1, 1, true),
                (-1, 0, true),
                (-1, -1, true),
                (0, -1, true),
                (1, -1, true),
            ],
            PieceType::Bishop => vec![(-1, 1, true), (1, 1, true), (-1, -1, true), (1, -1, true)],
            PieceType::Knight => vec![
                (-1, 2, false),
                (1, 2, false),
                (2, 1, false),
                (2, -1, false),
                (-1, -2, false),
                (1, -2, false),
                (-2, 1, false),
                (-2, -1, false),
            ],
            PieceType::Rook => vec![(-1, 0, true), (1, 0, true), (0, -1, true), (0, 1, true)],
            PieceType::Pawn => vec![(-1, 0, false), (1, 0, false)],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    #[test]
    fn get_piece_move_directions() {
        assert_eq!(
            Piece::new(Color::White, PieceType::Rook).title.directions()[0],
            (-1, 0, true)
        );
    }
}

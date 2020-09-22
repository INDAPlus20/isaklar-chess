use std::fmt;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn forward(&self) -> i32 {
        if self == &Color::White {
            1
        } else {
            -1
        }
    }
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Piece {
    color: Color,
    title: PieceType,
    has_moved: bool,
}

impl Piece {
    pub fn new(color: Color, title: PieceType) -> Piece {
        Piece {
            color,
            title,
            has_moved: false,
        }
    }

    pub fn title(&self) -> PieceType {
        self.title
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn has_moved(&self) -> bool {
        self.has_moved
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
    // Set the moves for all PieceTypes as offsets from current position (horizontal, vertical, repeating)

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
            PieceType::Pawn => vec![(0, 1, false), (0, 2, false), (-1, 1, false), (-1, 1, false)],
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self.title() {
            PieceType::King    => if self.color == Color::White {"K  "} else {"k  "},
            PieceType::Queen   => if self.color == Color::White {"Q  "} else {"q  "},
            PieceType::Bishop  => if self.color == Color::White {"B  "} else {"b  "},
            PieceType::Knight  => if self.color == Color::White {"Kn "} else {"kn "},
            PieceType::Rook    => if self.color == Color::White {"R  "} else {"r  "},
            PieceType::Pawn    => if self.color == Color::White {"P  "} else {"p  "},
        })
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

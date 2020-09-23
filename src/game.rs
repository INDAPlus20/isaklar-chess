use crate::piece::*;
use std::collections::HashMap;
use std::fmt;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    BlackCheck,
    WhiteCheck,
    GameOver,
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    active_color: Color,
    board: [Option<Piece>; 64],
    possible_moves: HashMap<String, Vec<String>>,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            active_color: Color::White,
            board: generate_board(),
            possible_moves: calculate_all_possible_moves(generate_board()),
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        let moves_option: Option<&Vec<String>> = self.possible_moves.get(&from);

        // If move is legal
        if moves_option.is_some() && moves_option.unwrap().contains(&to) {
            let piece = self.board[as_coordinate(&from)];
            // If capturing move
            if self.board[as_coordinate(&to)].is_some() {
                self.board[as_coordinate(&to)] = piece;
                self.board[as_coordinate(&from)].take();
            } else {
                self.board[as_coordinate(&to)] = piece;
                self.board[as_coordinate(&from)].take();
            }
            // Move has been made, now switch colors
            self.active_color = match self.active_color {
                Color::Black => Color::White,
                Color::White => Color::Black,
            };

            // Calculate all moves for new boardstate
            self.possible_moves = calculate_all_possible_moves(self.board);

            // If board is in check
            if let Some(state) = board_in_check(self.board, &self.possible_moves) {
                self.state = state;
                return Some(state);
            }

            // Not check
            self.state = GameState::InProgress;
            return Some(GameState::InProgress);
        }

        // No move has been made
        Some(self.state)
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }

    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.

    pub fn get_possible_moves(&self, position: String) -> Option<Vec<String>> {
        match self.possible_moves.get(&position).is_some() {
            true => Some(self.possible_moves.get(&position).unwrap().clone()),
            false => None,
        }
    }

    pub fn get_board(&self) -> &[Option<Piece>; 64] {
        &self.board
    }
}
fn move_piece(mut board: [Option<Piece>; 64], from: String, to: String) -> [Option<Piece>; 64] {
    let piece = board[as_coordinate(&from)];
    // If capturing move
    if board[as_coordinate(&to)].is_some() {
        board[as_coordinate(&to)] = piece;
        board[as_coordinate(&from)].take();
    } else {
        board[as_coordinate(&to)] = piece;
        board[as_coordinate(&from)].take();
    }

    board
}

fn board_in_check(
    board: [Option<Piece>; 64],
    possible_moves: &HashMap<String, Vec<String>>,
) -> Option<GameState> {
    let black_king_position = board
        .iter()
        .position(|&r| {
            r.is_some()
                && r.unwrap().title() == PieceType::King
                && r.unwrap().color() == Color::Black
        })
        .unwrap();
    // Do a recursive search of the nearby tiles to see if anyone checks the king
    let directions = PieceType::Queen.directions();
    for (file_move, rank_move, _) in directions {
        for i in 1..8 {
            // Calculate move coordinate
            let temp_move = (i * (file_move + rank_move * 8)) + (black_king_position as i32);

            // If the move is in bounds of the board
            if move_in_bounds(temp_move, black_king_position, file_move) {
                // Check if occupied
                if let Some(piece) = board[temp_move as usize] {
                    // Check occupying piece
                    if piece.color() == Color::Black {
                        continue;
                    } else {
                        if file_move == 0 || rank_move == 0 {
                            match piece.title() {
                                PieceType::Queen => return Some(GameState::BlackCheck),
                                PieceType::Rook => return Some(GameState::BlackCheck),
                                _ => continue
                            }
                        } else if rank_move == -1 && piece.title() == PieceType::Pawn {
                            return Some(GameState::BlackCheck)
                        } else {
                            match piece.title() {
                                PieceType::Queen => return Some(GameState::BlackCheck),
                                PieceType::Bishop => return Some(GameState::BlackCheck),
                                _ => continue
                            }
                        }
                    }
                } else {

                    continue;
                }
            } else {
                continue;
            }
        }
    }

    
    ////////////////////////////////////////////////////////////////Check for knight pieces


    let white_king_position = board
        .iter()
        .position(|&r| {
            r.is_some()
                && r.unwrap().title() == PieceType::King
                && r.unwrap().color() == Color::White
        })
        .unwrap();

    for (_piece, move_list) in possible_moves.iter() {
        for square in move_list {
            if let Some(piece) = board[as_coordinate(square)] {
                if piece.title() == PieceType::King {
                    match piece.color() {
                        Color::White => return Some(GameState::WhiteCheck),
                        Color::Black => return Some(GameState::BlackCheck),
                    }
                }
            }
        }
    }
    None
}

fn calculate_all_possible_moves(board: [Option<Piece>; 64]) -> HashMap<String, Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for square_index in 0..64 {
        if board[square_index].is_none() {
            continue;
        }
        // Calculate moves for the given piece
        let moves = calculate_possible_moves(&board, as_standard_notation(&square_index));
        if moves.is_some() {
            map.insert(as_standard_notation(&square_index), moves.unwrap());
        }
    }
    map
}

// Calculates the possible moves for a piece
pub fn calculate_possible_moves(
    board: &[Option<Piece>; 64],
    position: String,
) -> Option<Vec<String>> {
    let position = as_coordinate(&position);
    let piece = board[position].unwrap();
    let directions = piece.title().directions();

    let mut moves: Vec<String> = Vec::new();

    if piece.title() == PieceType::Pawn {
        // Pawn moves
        for (file_move, rank_move, _) in directions {
            let temp_move =
                (file_move + rank_move * 8 * piece.color().forward()) + (position as i32);

            if move_in_bounds(temp_move, position, file_move) {
                if board[temp_move as usize].is_some() {
                    // Diagonal move
                    if board[temp_move as usize].unwrap().color() != piece.color() && file_move != 0
                    {
                        moves.push(as_standard_notation(&(temp_move as usize)));
                    }
                // Straight move
                } else if file_move == 0 && rank_move == 2 && !piece.has_moved() {
                    moves.push(as_standard_notation(&(temp_move as usize)));
                } else if file_move == 0 {
                    moves.push(as_standard_notation(&(temp_move as usize)));
                }
            }
        }
    } else if directions[0].2 {
        // If the moves are repeating i.e Queen, Bishop, Rook

        for (file_move, rank_move, _) in directions {
            for i in 1..8 {
                // Calculate move coordinate
                let temp_move = (i * (file_move + rank_move * 8)) + (position as i32);

                // If the move is in bounds of the board
                if move_in_bounds(temp_move, position, file_move) {
                    // Check if occupied
                    if board[temp_move as usize].is_some() {
                        // Check occupying piece
                        if board[temp_move as usize].unwrap().color() == piece.color() {
                            break;
                        } else {
                            moves.push(as_standard_notation(&(temp_move as usize)));
                            break;
                        }
                    } else {
                        moves.push(as_standard_notation(&(temp_move as usize)));
                        continue;
                    }
                } else {
                    continue;
                }
            }
        }
    } else {
        // If the moves are not repeating i.e King, Knight
        // TODO
        for (file_move, rank_move, _) in directions {
            // Calculate move coordinate
            let temp_move = (file_move + rank_move * 8) + (position as i32);
            if move_in_bounds(temp_move, position, file_move) {
                if board[temp_move as usize].is_some() {
                    // Check occupying piece
                    if board[temp_move as usize].unwrap().color() != piece.color() {
                        moves.push(as_standard_notation(&(temp_move as usize)));
                    }
                } else {
                    moves.push(as_standard_notation(&(temp_move as usize)));
                }
            }
        }
    }

    if moves.len() > 0 {
        Some(moves)
    } else {
        None
    }
}

pub fn move_in_bounds(to: i32, from: usize, file_move: i32) -> bool {
    0 <= file_move + (from % 8) as i32 && file_move + (from % 8) as i32 <= 7 && to <= 63 && to >= 0
}

// Converts standard chess notation into indexable values
pub fn as_coordinate(input: &String) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let file = match chars[0] {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        _ => 0,
    };

    let rank: u32 = (chars[1].to_digit(10).unwrap() - 1) * 8;
    (rank + file) as usize
}

pub fn as_standard_notation(input: &usize) -> String {
    let mut file = match input % 8 {
        0 => "A".to_string(),
        1 => "B".to_string(),
        2 => "C".to_string(),
        3 => "D".to_string(),
        4 => "E".to_string(),
        5 => "F".to_string(),
        6 => "G".to_string(),
        7 => "H".to_string(),
        _ => "A".to_string(),
    };

    let rank = ((input / 8) + 1).to_string();

    file.push_str(&rank);
    file
}

fn generate_board() -> [Option<Piece>; 64] {
    [
        Some(Piece::new(Color::White, PieceType::Rook)),
        Some(Piece::new(Color::White, PieceType::Knight)),
        Some(Piece::new(Color::White, PieceType::Bishop)),
        Some(Piece::new(Color::White, PieceType::Queen)),
        Some(Piece::new(Color::White, PieceType::King)),
        Some(Piece::new(Color::White, PieceType::Bishop)),
        Some(Piece::new(Color::White, PieceType::Knight)),
        Some(Piece::new(Color::White, PieceType::Rook)),
        Some(Piece::new(Color::White, PieceType::Pawn)),
        Some(Piece::new(Color::White, PieceType::Pawn)),
        Some(Piece::new(Color::White, PieceType::Pawn)),
        Some(Piece::new(Color::White, PieceType::Pawn)),
        Some(Piece::new(Color::White, PieceType::Pawn)),
        Some(Piece::new(Color::White, PieceType::Pawn)),
        Some(Piece::new(Color::White, PieceType::Pawn)),
        Some(Piece::new(Color::White, PieceType::Pawn)),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(Piece::new(Color::Black, PieceType::Pawn)),
        Some(Piece::new(Color::Black, PieceType::Pawn)),
        Some(Piece::new(Color::Black, PieceType::Pawn)),
        Some(Piece::new(Color::Black, PieceType::Pawn)),
        Some(Piece::new(Color::Black, PieceType::Pawn)),
        Some(Piece::new(Color::Black, PieceType::Pawn)),
        Some(Piece::new(Color::Black, PieceType::Pawn)),
        Some(Piece::new(Color::Black, PieceType::Pawn)),
        Some(Piece::new(Color::Black, PieceType::Rook)),
        Some(Piece::new(Color::Black, PieceType::Knight)),
        Some(Piece::new(Color::Black, PieceType::Bishop)),
        Some(Piece::new(Color::Black, PieceType::Queen)),
        Some(Piece::new(Color::Black, PieceType::King)),
        Some(Piece::new(Color::Black, PieceType::Bishop)),
        Some(Piece::new(Color::Black, PieceType::Knight)),
        Some(Piece::new(Color::Black, PieceType::Rook)),
    ]
}

/// Implement print routine for Game.
///
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        write!(f, "A  B  C  D  E  F  G  H \n")?;
        for rank in (0..8).rev() {
            for file in 0..8 {
                if self.get_board()[file + rank * 8].is_some() {
                    write!(f, "{}", self.get_board()[file + rank * 8].unwrap())?;
                } else {
                    write!(f, "*  ")?;
                }
            }
            write!(f, "{} \n", rank + 1)?;
        }
        write!(f, "\n")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Color;
    use super::GameState;
    use super::Piece;
    use super::PieceType;
    use super::*;
    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {
        let game = Game::new();

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    // Test if board has values that can be read
    #[test]
    fn readable_board_pieces_after_init() {
        let game = Game::new();

        assert_eq!(
            game.get_board()[0].unwrap(),
            Piece::new(Color::White, PieceType::Rook)
        );
    }

    #[test]
    fn convert_input_to_coordinates() {
        assert_eq!(as_coordinate(&"B4".to_string()), 25)
    }

    #[test]
    fn get_piece_directions_after_board_init() {
        let game = Game::new();

        assert_eq!(
            game.get_board()[0].unwrap().title().directions()[0],
            (-1, 0, true)
        );
    }

    #[test]
    fn convert_coordinates_to_standard_notation() {
        assert_eq!(as_standard_notation(&(25 as usize)), "B4".to_string());
    }

    #[test]
    fn get_possible_moves_from_board_init() {
        let game = Game::new();

        assert_eq!(
            game.get_possible_moves(String::from("B1")).unwrap(),
            vec![String::from("A3"), String::from("C3")]
        );
    }

    #[test]
    fn get_pawn_possible_moves() {
        let game = Game::new();

        assert_eq!(
            game.get_possible_moves(String::from("B2")).unwrap(),
            vec![String::from("B3"), String::from("B4")]
        );
    }

    #[test]
    fn printable_board() {
        let game = Game::new();

        print!("{}", game);
    }

    #[test]
    fn make_moves() {
        let mut game = Game::new();
        game.make_move(String::from("D2"), String::from("D3"));
        print!("{}", game);
        game.make_move(String::from("D7"), String::from("D5"));
        print!("{}", game);
        game.make_move(String::from("A2"), String::from("A3"));
        print!("{}", game);
    }

    #[test]
    fn is_check_possible() {
        let mut game = Game::new();
        game.make_move(String::from("E2"), String::from("E3"));
        print!("{}", game);
        game.make_move(String::from("A7"), String::from("A5"));
        print!("{}", game);
        game.make_move(String::from("D1"), String::from("H5"));
        print!("{}", game);
        game.make_move(String::from("A8"), String::from("A7"));
        print!("{}", game);
        game.make_move(String::from("F1"), String::from("C4"));
        print!("{}", game);
        game.make_move(String::from("B7"), String::from("B6"));
        print!("{}", game);
        game.make_move(String::from("H5"), String::from("F7"));
        print!("{}", game);
        assert_eq!(game.get_game_state(), GameState::BlackCheck);
    }
    #[test]
    fn can_king_check_itself() {
        let mut game = Game::new();
        game.make_move(String::from("E2"), String::from("E3"));
        print!("{}", game);
        game.make_move(String::from("A7"), String::from("A5"));
        print!("{}", game);
        game.make_move(String::from("D1"), String::from("H5"));
        print!("{}", game);
        game.make_move(String::from("A8"), String::from("A7"));
        print!("{}", game);
        game.make_move(String::from("F1"), String::from("C4"));
        print!("{}", game);
        game.make_move(String::from("B7"), String::from("B6"));
        print!("{}", game);
        game.make_move(String::from("H5"), String::from("F7"));
        print!("{}", game);
        game.make_move(String::from("E8"), String::from("F7"));
        print!("{}", game);
    }
}

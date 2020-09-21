use crate::piece::*;
use std::fmt;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
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
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            active_color: Color::White,
            board: generate_board(),
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        None
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

    /// TODO
    pub fn get_possible_moves(&self, position: String) -> Option<Vec<String>> {
        let position = as_coordinate(&position);
        let piece = self.board[position].unwrap();
        let directions = piece.title().directions();
        println!("{:?}", directions);

        let mut moves: Vec<String> = Vec::new();

        if piece.title() == PieceType::Pawn {

            for (file_move, rank_move, _) in directions {

                let temp_move = (file_move + rank_move * 8 * piece.color().forward()) + (position as i32);

                if move_in_bounds(temp_move, position, file_move){

                    if self.board[temp_move as usize].is_some() {

                        // Check occupying piece
                        if self.board[temp_move as usize].unwrap().color() != self.active_color && file_move != 0 {
                            moves.push(as_standard_notation(&(temp_move as usize)));
                            println! {"Enemy piece"};
                        }
                    } else if file_move == 0 && rank_move == 2 && !piece.has_moved() {
                        moves.push(as_standard_notation(&(temp_move as usize)));
                        println! {"No piece"};

                    } else if file_move == 0{
                        moves.push(as_standard_notation(&(temp_move as usize)));
                        println! {"No piece"};
                    }
                }
            }

        } else if directions[0].2 {
            // If the moves are repeating i.e Queen, Bishop, Rook

            for (file_move, rank_move, _) in directions {
                for i in 0..8 {
                    // Calculate move coordinate
                    let temp_move = (i * (file_move + rank_move * 8)) + (position as i32);

                    // If the move is in bounds of the board
                    if move_in_bounds(temp_move, position, file_move){
                        // Check if occupied
                        if self.board[temp_move as usize].is_some() {
                            // Check occupying piece
                            if self.board[temp_move as usize].unwrap().color() == self.active_color
                            {
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
                println!("temp move: {:?}", temp_move);
                if move_in_bounds(temp_move, position, file_move){
                    if self.board[temp_move as usize].is_some() {
                        // Check occupying piece
                        if self.board[temp_move as usize].unwrap().color() != self.active_color {
                            moves.push(as_standard_notation(&(temp_move as usize)));
                            println! {"Enemy piece"};
                        }
                    } else {
                        moves.push(as_standard_notation(&(temp_move as usize)));
                        println! {"No piece'"};
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

    pub fn get_board(&self) -> &[Option<Piece>; 64] {
        &self.board
    }
}

pub fn move_in_bounds(to: i32, from: usize, file_move: i32) -> bool{
    0 <= file_move + (from % 8) as i32
    && file_move + (from % 8) as i32 <= 7
    && to <= 63
    && to >= 0
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
    println!("Rank: {:?}", rank);
    println!("File: {:?}", file);
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
        Some(Piece::new(Color::White, PieceType::King)),
        Some(Piece::new(Color::White, PieceType::Queen)),
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
        Some(Piece::new(Color::Black, PieceType::King)),
        Some(Piece::new(Color::Black, PieceType::Queen)),
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
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */

        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Color;
    use super::*;
    use super::GameState;
    use super::Piece;
    use super::PieceType;
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
        assert_eq!(as_coordinate(&"B3".to_string()), 25)
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
        assert_eq!(as_standard_notation(&(25 as usize)), "B3".to_string());
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
    fn get_pawn_possible_moves(){
        let game = Game::new();

        assert_eq!(
            game.get_possible_moves(String::from("B2")).unwrap(),
            vec![String::from("B3"), String::from("B4")]
        );
    }
}

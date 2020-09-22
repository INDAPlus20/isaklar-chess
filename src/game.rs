use crate::piece::*;
use std::fmt;
use std::collections::HashMap;
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
    possible_moves: HashMap<String, Vec<String>>
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            active_color: Color::White,
            board: generate_board(),
            possible_moves: Game::calculate_all_possible_moves(generate_board())
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        // If move is legal
        if get_possible_moves(self.board, from).unwrap().contains(&to) {
            // If capturing move
            if self.board[as_coordinate(to)].is_some(){
                
                make_capturing_move(self, from, to);

            }
        } else {
            None
        }
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }


    fn calculate_all_possible_moves(board: [Option<Piece>; 64] ) -> HashMap<String, Vec<String>>{
        for square_index in 0..64{
            if board[square_index].is_none(){
                continue;
            }

            let moves = calculate_possible_moves(&board, as_standard_notation(&square_index));
            if moves.is_some() {
                
            }

        }

        vec![(String::from(""), None)]
    }
    /// If a piece is standing on the given tile, return all possible
    /// new positions of that piece. Don't forget to the rules for check.
    ///
    /// (optional) Don't forget to include en passent and castling.

    /// TODO
    pub fn get_possible_moves(&self, position: String) -> Option<Vec<String>>{
        self.possible_moves()
    }

    pub fn get_board(&self) -> &[Option<Piece>; 64] {
        &self.board
    }


    fn regular_move(&mut self, from: String, to: String){
        //self.board[from.]
    }

    fn capturing_move(&self, from: String, to: String){

    }
}

pub fn calculate_possible_moves(board: &[Option<Piece>; 64], position: String) -> Option<Vec<String>> {
    let position = as_coordinate(&position);
    let piece = board[position].unwrap();
    let directions = piece.title().directions();
    println!("{:?}", directions);

    let mut moves: Vec<String> = Vec::new();

    if piece.title() == PieceType::Pawn {
        // Pawn moves
        for (file_move, rank_move, _) in directions {
            let temp_move =
                (file_move + rank_move * 8 * piece.color().forward()) + (position as i32);

            if move_in_bounds(temp_move, position, file_move) {
                if board[temp_move as usize].is_some() {
                    // Diagonal move
                    if board[temp_move as usize].unwrap().color() != piece.color()
                        && file_move != 0
                    {
                        moves.push(as_standard_notation(&(temp_move as usize)));
                        println! {"Enemy piece"};
                    }
                // Straight move 
                } else if file_move == 0 && rank_move == 2 && !piece.has_moved() {
                    moves.push(as_standard_notation(&(temp_move as usize)));
                    println! {"No piece"};
                } else if file_move == 0 {
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
                if move_in_bounds(temp_move, position, file_move) {
                    // Check if occupied
                    if board[temp_move as usize].is_some() {
                        // Check occupying piece
                        if board[temp_move as usize].unwrap().color() == piece.color()
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
            if move_in_bounds(temp_move, position, file_move) {
                if board[temp_move as usize].is_some() {
                    // Check occupying piece
                    if board[temp_move as usize].unwrap().color() != piece.color() {
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
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        write!(f, "A  B  C  D  E  F  G  H \n")?;
        for rank in (0..8).rev(){
            for file in 0..8{
                
                if self.get_board()[file + rank * 8].is_some(){
                    write!(f,"{}", self.get_board()[file + rank * 8].unwrap())?;
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
    fn get_pawn_possible_moves() {
        let game = Game::new();

        assert_eq!(
            game.get_possible_moves(String::from("B2")).unwrap(),
            vec![String::from("B3"), String::from("B4")]
        );
    }

    #[test]
    fn printable_board(){
        let game = Game::new();

        print!("{}", game);
        
    }
    
}

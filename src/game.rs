use crate::piece::*;
use std::collections::HashMap;
use std::fmt;
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    BlackCheck,
    WhiteCheck,
    BlackCheckMate,
    WhiteCheckMate,
    Tie,
}
fn color_check(color: Color) -> GameState {
    match color {
        Color::Black => GameState::BlackCheck,
        Color::White => GameState::WhiteCheck,
    }
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

    pub fn new_with_board(board: [Option<Piece>; 64]) -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            active_color: Color::White,
            board,
            possible_moves: calculate_all_possible_moves(board),
        }
    }

    /// If the current game state is InProgress and the move is legal,
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, from: String, to: String) -> Option<GameState> {
        let moves_option: Option<&Vec<String>> = self.possible_moves.get(&from);

        // If move is legal
        if moves_option.is_some() && moves_option.unwrap().contains(&to) && self.state == GameState::InProgress &&  self.board[as_coordinate(&from)].unwrap().color() == self.active_color{
            let mut piece = self.board[as_coordinate(&from)].unwrap();

            // Piece has Moved
            piece.set_has_moved();

            // If capturing move
            if self.board[as_coordinate(&to)].is_some() {
                self.board[as_coordinate(&to)] = Some(piece);
                self.board[as_coordinate(&from)].take();
            } else {
                self.board[as_coordinate(&to)] = Some(piece);
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
            if let Some(state) = board_in_check(self.board) {
                // If checkmate
                match state {
                    GameState::WhiteCheck => {
                        if self
                            .possible_moves
                            .get(&as_standard_notation(&find_kings(&self.board)[0]))
                            .is_none()
                        {
                            return Some(GameState::WhiteCheckMate);
                        }
                    }
                    GameState::BlackCheck => {
                        if self
                            .possible_moves
                            .get(&as_standard_notation(&find_kings(&self.board)[1]))
                            .is_none()
                        {
                            return Some(GameState::BlackCheckMate);
                        }
                    }
                    _ => (),
                }
                self.state = state;
                return Some(state);
            }

            // If tie
            let kings = find_kings(&self.board);
            for king in kings{
                if self
                .possible_moves
                .get(&as_standard_notation(&king))
                .is_none(){
                    self.state = GameState::Tie;
                    return Some(GameState::Tie);
                }
            }
            

            // Not check or tie
            self.state = GameState::InProgress;
            return Some(GameState::InProgress);
        }

        // Impossible move
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

// Returns a new board with the given move
fn board_from_move(mut board: [Option<Piece>; 64], from: usize, to: usize) -> [Option<Piece>; 64] {
    let piece = board[from];
    // If capturing move
    if board[to].is_some() {
        board[to] = piece.clone();
        board[from].take();
    } else {
        board[to] = piece.clone();
        board[from].take();
    }

    board
}

fn find_kings(board: &[Option<Piece>; 64]) -> Vec<usize> {
    let mut white_king_position = 0;
    for square in 0..64 {
        if let Some(piece) = board[square] {
            if piece.title() == PieceType::King && piece.color() == Color::White {
                white_king_position = square;
            }
        }
    }

    let mut black_king_position = 0;
    for square in 0..64 {
        if let Some(piece) = board[square] {
            if piece.title() == PieceType::King && piece.color() == Color::Black {
                black_king_position = square;
            }
        }
    }

    vec![white_king_position, black_king_position]
}

fn board_in_check(board: [Option<Piece>; 64]) -> Option<GameState> {
    let kings_positions = find_kings(&board);

    // Do a recursive search of the nearby tiles to see if anyone checks the king
    let directions = PieceType::Queen.directions();

    for position in kings_positions {
        let king = board[position].unwrap();
        let check = match king.color() {
            Color::Black => GameState::BlackCheck,
            Color::White => GameState::WhiteCheck,
        };
        for (file_move, rank_move, _) in directions.clone() {
            for i in 1..8 {
                // Calculate move coordinate
                let temp_move = (i * (file_move + rank_move * 8)) + (position as i32);

                // If the move is in bounds of the board
                if move_in_bounds(temp_move, position, file_move) {
                    // Check if occupied
                    if let Some(piece) = board[temp_move as usize] {
                        // Check occupying piece
                        if piece.color() == king.color() {
                            break;
                        } else {
                            if file_move == 0 || rank_move == 0 {
                                match piece.title() {
                                    PieceType::Queen => return Some(check),
                                    PieceType::Rook => return Some(check),
                                    _ => break,
                                }
                            } else if rank_move*i == king.color().forward() 
                                && piece.title() == PieceType::Pawn
                            {
                                return Some(check);
                            } else {
                                match piece.title() {
                                    PieceType::Queen => return Some(check),
                                    PieceType::Bishop => return Some(check),
                                    _ => break,
                                }
                            }
                        }
                    }
                }
            }
        }

        // Check for knight pieces
        let knight_directions = PieceType::Knight.directions();
        for (file_move, rank_move, _) in knight_directions {
            // Calculate move coordinate
            let temp_move = (file_move + rank_move * 8) + (position as i32);

            // If the move is in bounds of the board
            if move_in_bounds(temp_move, position, file_move) {
                // Check if occupied
                if let Some(piece) = board[temp_move as usize] {
                    // Check occupying piece
                    if piece.color() == king.color() {
                        continue;
                    } else if piece.title() == PieceType::Knight {
                        return Some(check);
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
    let board: [Option<Piece>; 64] = board.clone();
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
                if board_in_check(board_from_move(board.clone(), position, temp_move as usize))
                    != Some(color_check(piece.color()))
                {
                    if board[temp_move as usize].is_some() {
                        // Diagonal move
                        if board[temp_move as usize].unwrap().color() != piece.color()
                            && file_move != 0
                        {
                            moves.push(as_standard_notation(&(temp_move as usize)));
                        }
                    // Straight move
                    } else if file_move == 0 && rank_move == 1 {
                        moves.push(as_standard_notation(&(temp_move as usize)));
                    } else if file_move == 0
                        && rank_move == 2
                        && !piece.has_moved()
                    {
                        moves.push(as_standard_notation(&(temp_move as usize)));
                    }
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
                    if board_in_check(board_from_move(board.clone(), position, temp_move as usize))
                        != Some(color_check(piece.color()))
                    {
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
        }
    } else {
        // If the moves are not repeating i.e King, Knight
        // TODO
        for (file_move, rank_move, _) in directions {
            // Calculate move coordinate
            let temp_move = (file_move + rank_move * 8) + (position as i32);
            if move_in_bounds(temp_move, position, file_move) {
                if board_in_check(board_from_move(board.clone(), position, temp_move as usize))
                    != Some(color_check(piece.color()))
                {
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
        Some(Piece::new(Color::White, PieceType::Rook, false)),
        Some(Piece::new(Color::White, PieceType::Knight, false)),
        Some(Piece::new(Color::White, PieceType::Bishop, false)),
        Some(Piece::new(Color::White, PieceType::Queen, false)),
        Some(Piece::new(Color::White, PieceType::King, false)),
        Some(Piece::new(Color::White, PieceType::Bishop, false)),
        Some(Piece::new(Color::White, PieceType::Knight, false)),
        Some(Piece::new(Color::White, PieceType::Rook, false)),
        Some(Piece::new(Color::White, PieceType::Pawn, false)),
        Some(Piece::new(Color::White, PieceType::Pawn, false)),
        Some(Piece::new(Color::White, PieceType::Pawn, false)),
        Some(Piece::new(Color::White, PieceType::Pawn, false)),
        Some(Piece::new(Color::White, PieceType::Pawn, false)),
        Some(Piece::new(Color::White, PieceType::Pawn, false)),
        Some(Piece::new(Color::White, PieceType::Pawn, false)),
        Some(Piece::new(Color::White, PieceType::Pawn, false)),
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
        Some(Piece::new(Color::Black, PieceType::Pawn, false)),
        Some(Piece::new(Color::Black, PieceType::Pawn, false)),
        Some(Piece::new(Color::Black, PieceType::Pawn, false)),
        Some(Piece::new(Color::Black, PieceType::Pawn, false)),
        Some(Piece::new(Color::Black, PieceType::Pawn, false)),
        Some(Piece::new(Color::Black, PieceType::Pawn, false)),
        Some(Piece::new(Color::Black, PieceType::Pawn, false)),
        Some(Piece::new(Color::Black, PieceType::Pawn, false)),
        Some(Piece::new(Color::Black, PieceType::Rook, false)),
        Some(Piece::new(Color::Black, PieceType::Knight, false)),
        Some(Piece::new(Color::Black, PieceType::Bishop, false)),
        Some(Piece::new(Color::Black, PieceType::Queen, false)),
        Some(Piece::new(Color::Black, PieceType::King, false)),
        Some(Piece::new(Color::Black, PieceType::Bishop, false)),
        Some(Piece::new(Color::Black, PieceType::Knight, false)),
        Some(Piece::new(Color::Black, PieceType::Rook, false)),
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
            Piece::new(Color::White, PieceType::Rook, false)
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
    fn king_cant_check_itself() {
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
        assert_eq!(
            game.make_move(String::from("E8"), String::from("F7")),
            Some(GameState::BlackCheck)
        );
    }

    #[test]
    fn check_mate_possible() {
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
        assert_eq!(
            game.make_move(String::from("H5"), String::from("F7")),
            Some(GameState::BlackCheckMate)
        );
        print!("{}", game);
    }

    #[test]
    fn pawns_cant_double_move_twice() {
        let mut game = Game::new();
        game.make_move(String::from("E2"), String::from("E4"));
        print!("{}", game);
        game.make_move(String::from("A7"), String::from("A5"));
        print!("{}", game);
        assert_eq!(game.make_move(String::from("E4"), String::from("E6")), None);
        print!("{}", game);
    }

    #[test]
    fn pawns_can_attack_diagonally(){
        let mut game = Game::new();
        game.make_move(String::from("E2"), String::from("E4"));
        print!("{}", game);
        game.make_move(String::from("E4"), String::from("E5"));
        print!("{}", game);
        game.make_move(String::from("E5"), String::from("E6"));
        print!("{}", game);
        assert_eq!(game.make_move(String::from("E6"), String::from("F7")), Some(GameState::BlackCheck));
        print!("{}", game);
        
    }

    #[test]
    fn tie_is_possible(){
        let mut game = Game::new_with_board([
            Some(Piece::new(Color::White, PieceType::King, false)),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::new(Color::Black, PieceType::Queen, false)),
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::new(Color::Black, PieceType::Rook, false)),
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
            Some(Piece::new(Color::Black, PieceType::King, false)),
            None,
        ]);
        
        
        game.make_move(String::from("B3"), String::from("B2"));
        print!("{}", game);
        assert_eq!(game.get_game_state(), GameState::Tie);

    }
}

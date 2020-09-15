use std::fmt;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
    
}

/* IMPORTANT:
 * - Document well!
 * - Write well structured and clean code!
 */

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    active_color: Color,
    board: Vec<Option<Piece>>
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            active_color: Color::White,
            board: generate_board()
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
    pub fn get_possible_moves(&self, _position: String) -> Option<Vec<String>> {
        None
    }

    pub fn get_board(&self) -> &Vec<Option<Piece>> {
        &self.board
    }

    // Converts standard chess notation into indexable values
    pub fn as_coordinate(input: String) -> u32 {
        let chars: Vec<char> = input.chars().collect();
        let rank = match chars[0] {
            'A' => 0 * 8,
            'B' => 1 * 8,
            'C' => 2 * 8,
            'D' => 3 * 8,
            'E' => 4 * 8,
            'F' => 5 * 8,
            'G' => 6 * 8,
            'H' => 7 * 8,
            _ => 0
        };

        let file: u32 = chars[1].to_digit(10).unwrap();

        rank + file
    }

}


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Piece{
    color: Color,
    title: PieceType

}

impl Piece{
    pub fn new(color: Color, title: PieceType) -> Piece {
        Piece{
            color,
            title
        }
    }

    pub fn get_title(&self) -> &PieceType{
        &self.title
    }
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum PieceType{
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn

}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Color{
    Black,
    White
}


fn generate_board() -> Vec<Option<Piece>> {
    vec![
    Some(Piece::new(Color::White, PieceType::Rook)), Some(Piece::new(Color::White, PieceType::Knight)), Some(Piece::new(Color::White, PieceType::Bishop)), Some(Piece::new(Color::White, PieceType::King)), Some(Piece::new(Color::White, PieceType::Queen)), Some(Piece::new(Color::White, PieceType::Bishop)), Some(Piece::new(Color::White, PieceType::Knight)), Some(Piece::new(Color::White, PieceType::Rook)),
    Some(Piece::new(Color::White, PieceType::Pawn)), Some(Piece::new(Color::White, PieceType::Pawn)), Some(Piece::new(Color::White, PieceType::Pawn)), Some(Piece::new(Color::White, PieceType::Pawn)), Some(Piece::new(Color::White, PieceType::Pawn)), Some(Piece::new(Color::White, PieceType::Pawn)), Some(Piece::new(Color::White, PieceType::Pawn)), Some(Piece::new(Color::White, PieceType::Pawn)),
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    None, None, None, None, None, None, None, None,
    Some(Piece::new(Color::Black, PieceType::Pawn)), Some(Piece::new(Color::Black, PieceType::Pawn)), Some(Piece::new(Color::Black, PieceType::Pawn)), Some(Piece::new(Color::Black, PieceType::Pawn)), Some(Piece::new(Color::Black, PieceType::Pawn)), Some(Piece::new(Color::Black, PieceType::Pawn)), Some(Piece::new(Color::Black, PieceType::Pawn)), Some(Piece::new(Color::Black, PieceType::Pawn)),
    Some(Piece::new(Color::Black, PieceType::Rook)), Some(Piece::new(Color::Black, PieceType::Knight)), Some(Piece::new(Color::Black, PieceType::Bishop)), Some(Piece::new(Color::Black, PieceType::King)), Some(Piece::new(Color::Black, PieceType::Queen)), Some(Piece::new(Color::Black, PieceType::Bishop)), Some(Piece::new(Color::Black, PieceType::Knight)), Some(Piece::new(Color::Black, PieceType::Rook))
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
    use super::Game;
    use super::GameState;
    use super::Piece;
    use super::PieceType;
    use super::Color;
    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }

    // Test if board has values that can be read
    #[test]
    fn readable_board_pieces_after_init(){
        let game = Game::new();

        assert_eq!(game.get_board()[0].unwrap(), Piece::new(Color::White, PieceType::Rook) );
    }

    #[test]
    fn convert_input_to_coordinates(){
        

        assert_eq!(Game::as_coordinate("B3".to_string()), 11)
    }
    
}

# isaklar-chess
I have not implemented en passent, castling, and promotion. Everything else works fine (i think). If you need examples on how to use it, check the tests in `Game.rs`
### Enumerables
| **Enumerable** | **Values** | **Description** |
|----------------|------------|-----------------|
| `GameState`    | `InProgress`, `WhiteCheck`, `BlackCheck`, `WhiteCheckMate`, `BlackCheckMate`, `Tie` | Represents the state that a game can have. |
| `Color`       | `White`, `Black` | Represents the color of a chess piece. |
| `PieceType`    | `King`, `Queen`, `Bishop`, `Knight`, `Rook`, `Pawn` | Represents the type of a chess piece. |


### Structure `Game`
*As stated in the rust-task-3 assignment instructions.*

| **Function** | **Description** |
|--------------|-----------------|
| `pub fn new() -> Game` | Initialises a new board with pieces. |
| `new_with_board(board: [Option<Piece>; 64]) -> Game` | Initialises a new game with a specified board. |
| `pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState>` | If the current game state is `InProgress` and the move is legal, move a piece and return the resulting state of the game. Otherwise returns `None` |
| `pub fn set_promotion(&mut self, _piece: String) -> ()` | Promotion isn't implemented so this is useless |
| `pub fn get_game_state(&self) -> GameState` | Get the current game state. |
| `pub fn get_possible_moves(&self, _position: String) -> Optional<Vec<String>>` | If a piece is standing on the given tile, return all possible new positions of that piece. En passent and castling are not implemented. |
|`pub fn get_board(&self) -> &[Option<Piece>; 64]` | Get the current board |



Positions are given as strings with the format `"<file><rank>"`.

### Structure `Piece`
Contains three fields: `color`, `title` (PieceType) , and `has_moved`

| **Function** | **Description** |
|--------------|-----------------|
| `pub fn new(color: Color, title: PieceType, has_moved: bool) -> Piece `| Creates a new piece with the given parameters |
| `pub fn title(&self) -> PieceType` | Returns the `PieceType`, also called "title"|
| `pub fn color(&self) -> Color ` | Returns the `Color` | 
| `pub fn has_moved(&self) -> bool ` | If the piece has moved| 
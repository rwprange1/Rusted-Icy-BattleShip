use std::collections::HashMap;


// Defne the size of the game board as a constant 10x10
const BOARD_SIZE: usize = 10;

// an enum rep of the types of states a cell in the board can have
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CellState{
    // If a cell has nothing in it
    Empty,
    // If a cell have a ship in it
    Ship(ShipType),
    // If a cell has a hit in it
    Hit,
    // If a cell has miss in it
    Miss,
}

// an enum rep of the types of ships
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum ShipType{
    // boat of len 5
    Carrier,
    // boat of len 4
    BattleShip,
    // boat of len 3
    Cruiser,
    // boat of len 3
    Sub,
    // boat of len 2
    Destroyer,
}



// A simple struct to group information needed about moves together
pub struct Move {
    // type of command
    command: Commands,
    // the player who input it
    player: usize,
    // did the move result in a loss, and who lost
    kicked: (bool, usize),
    // the associated msg
    msg: String,
}

// A board which has a 2d array of cellstates 10x10, a hashmap of ships,
// the name of the owner of the board, the number of ships on the board, bool
// for if we are in test mode. Each client will have a board

pub struct Board{
    // The grid that represents this board.
    pub grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    // The ships on this board.
    ships: HashMap<ShipType, Ship>,
    // The name of the player that owns this board.
    name: String,
    // The number of ships that have not been sunk.
    num_ships: usize,
    // is the server running a test game
    test_mode: bool,
}

impl Default for Board{
    fn default() ->Board{
        Board{
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            ships: HashMap::new(),
            name: "".to_string(),
            num_ships: 5,
            test_mode: false,
            
        }
    }
}



// A ship struct which will keep track of how many times it
// has been hit and what type of ship it is
struct Ship{
    // The type of ship
    ship_type: ShipType,
    // how many times has the boat been hit
    hits: usize,
}

// A BattleShip struct which keeps track of the boards in the game,
// the number of terms. a list of players, a bool on whether the game has started
// the number of ships to be added to the boards. The game is built around the
// mutations of this structs properties
#[derive(Default)]
pub struct BattleShip{
    // All boards in the game.
    state: Vec<Board>,
    // The number of turns that have been played
    turns: usize,
    // The Names of all players. Repeated to make `contains` method faster.
    players: Vec<String>,
    // Whether the game has started
    started : bool,
    // the number of ships to play with
    num_ships: usize,
}

// the lists of commands
#[derive(Clone, PartialEq, Debug)]
pub enum Commands{
    // shoots at someone
    FIRE,
    // leave the game
    END,
    // starts the game
    START,
    // invalid
    INVALID,
}

impl CellState {
    pub fn to_string(&self) -> String {
        match &self {
            CellState::Empty => String::from("Empty"),
            CellState::Ship(_) => String::from("Ship"),
            CellState::Hit => String::from("Hit"),
            CellState::Miss => String::from("Miss"),
            _ => String::from(""),
        }
    }
}
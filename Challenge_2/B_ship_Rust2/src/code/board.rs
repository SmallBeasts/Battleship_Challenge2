use std::vec;
use crate::code::enums::Direction;

// This structure will be the main board per player
pub struct PlayBoard {
    playername: String,
    playernum: usize,
    ships: Vec<BoundingBox>,                    // This is used in create to store only ships
}

pub fn create_player() -> PlayBoard {
    PlayBoard {
        playername: String::new(),
        playernum: 0,
        ships: Vec::new(),
    }
}

impl PlayBoard {
    pub fn get_playername(&self) -> String {
        self.playername.clone()
    }

    pub fn set_playername(&mut self, name: String) {
        self.playername = name;
    }

    pub fn get_playernum(&self) -> usize {
        self.playernum
    }

    pub fn set_playernum(&mut self, num: usize) {
        self.playernum = num;
    }

    pub fn add_ship(&mut self, new_ship: BoundingBox) -> bool {
        for ship in self.ships {
            if ship.ship_id == new_ship.ship_id {       // Make sure that this new ship doesn't have the same id as another
                return false;
            }
        }
        self.ships.push(new_ship);
        true
    }

    // Check if any guess is a hit, if so return ship_id for the hit.
    pub fn handle_shot(&self, row: usize, col: usize) -> Option<usize>{
        for ship in self.ships {
            if ship.point_in_ship(row, col) {       // Hit
                return Some(ship.ship_id);
            }
        }
        None                                // Miss
    }

    

}


// This will be the main game data storage.  Boards will only be stored inside a Vector
pub struct GameData {
    rows: usize,
    cols: usize,
    player_count: usize,
    loaded: bool,
    interactive: bool,
    filename: String,
    smallestship: usize,
    largestship: usize,
    boards: Vec<PlayBoard>
}

impl GameData {
    pub fn set_interactive(&mut self, state: bool) {
        self.interactive = state;
    }

    pub fn get_interactive(&self) -> bool {
        self.interactive
    }

    pub fn get_loaded(&self) -> bool {
        self.loaded
    }

    pub fn set_loaded(&mut self, state: bool) {
        self.loaded = state;
    }

    pub fn get_filename(&self) -> String {
        self.filename.clone()
    }

    pub fn set_filename(&mut self, name: String) {
        self.filename = name;
    }

    pub fn get_shipsizes(&self) -> (usize, usize) {
        (self.smallestship, self.largestship)
    }

    pub fn set_shipsizes(&mut self, small: usize, large: Option<usize>) -> Result<(), &str> {
        let large_val = large.unwrap_or(small + 5);
        if small <= 1 || large_val <= 1 {
            return Err("Error: Ship sizes have to be positive and larger than 1");
        }
        self.smallestship = small;
        self.largestship = large_val;
        Ok(())
    }

    pub fn get_row_col(&self) -> (usize,usize) {
        (self.rows, self.cols)
    }
    // Set both rows and columns together if possible
    pub fn set_row_col(&mut self, row: usize, col: usize) {
        self.rows = row;
        self.cols = col;
    }
    
    // True means set row, false is cols
    pub fn set_row_or_col(&mut self, num: usize, row_col: bool) {
        if row_col {
            self.rows = num;
        } else {
            self.cols = num;
        }
    }

    pub fn get_playercount(&self) -> usize {
        self.player_count
    }

    pub fn set_playercount(&mut self, num: usize) {
        self.player_count = num;
    }

    pub fn increment_playercount(&mut self) {
        self.player_count += 1;
    }

    pub fn decrement_playercount(&mut self) -> Result<(), &str> {
        if self.player_count <= 0 {
            return Err("Error: Dropped below 0 players.");
        }
        self.player_count -= 1;
        Ok(())
    }

    pub fn get_boards_len(&self) -> usize {
        self.boards.len()
    }

    pub fn boards_add(&mut self, board: PlayBoard) {
        self.boards.push(board);
    }

    pub fn boards_get_last(&self) -> PlayBoard {
        Some(self.boards.last().cloned())
    }

    pub fn boards_pop_last(&mut self) -> PlayBoard {
        self.boards.pop()
    }

    pub fn boards_get_player(&self, playernum: usize) -> Option<PlayBoard> {
        if self.boards.len() < playernum {
            None
        }
        else {}
            Some(self.boards.get(playernum).cloned()?)
    }

    pub fn in_bounds(&self, row: usize, col: usize) -> bool{
        if row < self.rows && col < self.cols {
            return true;
        }
        false
    }
}

// This will create a new game board with empty Vec for boards
pub fn create_game() -> GameData {
    GameData {
        rows: 10,
        cols: 10,
        player_count: 1,
        loaded: false,
        interactive: false,
        filename: "".to_string(),
        smallestship: 2,
        largestship: 5,
        boards: Vec::new()
    }
}

pub struct ShipBoundingBox {
    pub ship_id: usize,
    pub start: (usize, usize),
    pub end: (usize, usize),
}
// Definition of ships and their locations
// Points are stored in column, row
impl ShipBoundingBox {

    pub fn new(
        ship_id: usize,
        start: (usize, usize),
        direction: Direction,
        board: &GameData,
    ) -> Option<ShipBoundingBox> {
        let tmp_end: (usize, usize) = (0,0);
        if direction == Direction::Vertical                  // Vertical ship
        {
            tmp_end = (start.0 + ship_id, start.1);
        } else {                                            // Horizontal ship
            tmp_end = (start.0, start.1 + ship_id);
        }
        if !board.in_bounds(start.0, start.1) && !board.in_bounds(tmp_end.0, tmp_end.1) {         // Valid for placement
            return None;
        }
        if start.0 != tmp_end.0 && start.1 != tmp_end.1 {               // Check for diagonal
            return None;
        }
        Some(ShipBoundingBox {
            ship_id: ship_id,
            start: (start.0, start.1),
            end: (tmp_end.0, tmp_end.1),
            })
    }

    // Check for a collision between ships
    pub fn collision(&self, other: &ShipBoundingBox) -> bool{
        (self.start.0 <= other.start.0 && self.end.0 >= other.start.0) &&
        (self.start.1 <= other.end.1 && self.end.1 >= other.start.1)
    }

    pub fn point_in_ship(&self, row: usize, col: usize) -> bool {
        if self.start.0 == self.end.0 { // Vertical ship
            (col == self.start.0) && (row >= self.start.1) && (row <= self.end.1) //Check col and row
        } else { // Horizontal ship
            (row == self.start.1) && (col >= self.start.0) && (col <= self.end.0) //Check row and col
        }
    }

}
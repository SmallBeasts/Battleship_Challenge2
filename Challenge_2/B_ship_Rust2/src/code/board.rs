use crate::code::enums::Direction;
use std::collections::HashSet;

// This structure will be the main board per player
pub struct PlayBoard {
    playername: String,
    playernum: usize,
    ships: Vec<ShipBoundingBox>,                    // This is used in create to store only ships
    ship_ids: HashSet<usize>,
}

impl Default for PlayBoard {
    fn default() -> Self {
        Self {
            playername: String::new(),
            playernum: 0,
            ships: Vec::new(),
            ship_ids: HashSet::new(),
        }
    }
}

impl PlayBoard {
    pub fn get_playername(&self) -> &String {
        &self.playername
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

    pub fn player_has_ship_id(&self, new_ship_id: usize) -> bool{
        self.ship_ids.contains(&new_ship_id)
    }

    pub fn add_ship(&mut self, new_ship: ShipBoundingBox) -> bool {
        if self.player_has_ship_id(new_ship.ship_id) {       // Make sure that this new ship doesn't have the same id as another
                return false;
        }

        self.ships.push(new_ship);
        true
    }

    pub fn return_ships(&self) -> &Vec<ShipBoundingBox> {
        &self.ships
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

    pub fn get_col_row(&self) -> (usize,usize) {
        (self.cols, self.rows)
    }
    // Set both rows and columns together if possible
    pub fn set_col_row(&mut self, col: usize, row: usize) {
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

    pub fn boards_get_last(&self) -> Option<&PlayBoard> {
        if self.boards.is_empty() {
            return None;
        }
        self.boards.last()
    }

    pub fn boards_pop_last(&mut self) -> Option<PlayBoard> {
        self.boards.pop()
    }

    pub fn boards_get_player(&self, playernum: usize) -> Option<&PlayBoard> {
        if playernum > self.boards.len() {
            return None;
        }
        self.boards.get(playernum)
    }

    pub fn in_bounds(&self, col: usize, row: usize) -> bool{
        row < self.rows && col < self.cols
    }

}

impl Default for GameData {
    fn default() -> Self {
        Self {
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
        start: (usize, usize),                              // Row, Col
        direction: Direction,
        board: &GameData,
        player: &PlayBoard,
    ) -> Option<ShipBoundingBox> {
        let tmp_end: (usize, usize) = (0,0);
        if direction == Direction::Vertical                  // Vertical ship
        {
            tmp_end = (start.1 + ship_id - 1, start.0);
        } else {                                            // Horizontal ship
            tmp_end = (start.1, start.0 + ship_id - 1);
        }
        if !board.in_bounds(start.0, start.1) || !board.in_bounds(tmp_end.0, tmp_end.1) {         // Valid for placement
            return None;
        }
        if start.0 != tmp_end.0 && start.1 != tmp_end.1 {               // Check for diagonal
            return None;
        }
        // Check for collision and ship_id duplicate
        if player.player_has_ship_id(ship_id) {             // Ship is a duplicate
            return None;
        }
        if player.ships.iter().any(|existing_ship| {                // Check for collision and overlap
            ShipBoundingBox::overlaps(&ShipBoundingBox{ship_id, start, end: tmp_end}, existing_ship) 
        }) {
            return None;
        } 
        
        Some(ShipBoundingBox {
            ship_id: ship_id,
            start: (start.0, start.1),
            end: (tmp_end.0, tmp_end.1),
            })
    }

// Check for a collision between ships
pub fn overlaps(ship1: &ShipBoundingBox, ship2: &ShipBoundingBox) -> bool {
    // Ship 1 is horizontal
    let ship1_horizontal = ship1.start.1 == ship1.end.1;
    // Ship 2 is horizontal
    let ship2_horizontal = ship2.start.1 == ship2.end.1;

    // Exit if both are horizontal and different rows
    if ship1_horizontal && ship2_horizontal {
        return ship1.start.0 != ship2.start.0; // Different columns
    }

    // Exit if both vertical but different rows
    if !ship1_horizontal && !ship2_horizontal {
        return ship1.start.1 != ship2.start.1; // Different rows
    }

    // At this point, one is vertical, and one is horizontal

    // Check if horizontal ship's column is within the vertical ship's column
    if ship1_horizontal {
        return (ship2.start.0 >= ship1.start.0 && ship2.start.0 <= ship1.end.0) &&
               (ship1.start.1 >= ship2.start.1 && ship1.start.1 <= ship2.end.1);
    } else {
        return (ship1.start.0 >= ship2.start.0 && ship1.start.0 <= ship2.end.0) &&
               (ship2.start.1 >= ship1.start.1 && ship2.start.1 <= ship1.end.1);
    }
}


    pub fn points_collision(&self, other_start: (usize, usize), other_end: (usize, usize)) -> bool {
        (self.start.1 <= other_start.1 && self.end.1 >= other_start.1) &&
        (self.start.0 <= other_end.0 && self.end.0 >= other_start.0)
    }

    pub fn point_in_ship(&self, row: usize, col: usize) -> bool {
        if self.start.0 == self.end.0 { // Vertical ship
            (col == self.start.0) && (row >= self.start.1) && (row <= self.end.1) //Check col and row
        } else { // Horizontal ship
            (row == self.start.1) && (col >= self.start.0) && (col <= self.end.0) //Check row and col
        }
    }

}
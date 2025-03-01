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

    pub fn get_largest_ship_id(&self) -> Option<usize> {
        self.ship_ids.iter().copied().max()
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

    pub fn check_collision(&self, start: (usize, usize), ship_size: usize, direction: Direction) -> bool {
        let (start_col, start_row) = start;

        for ship in &self.ships {
            if ship.overlap_possible(ship_size, start, &direction) {
                return true;
            }
        }

        false // No collision
    }

    // Check if any guess is a hit, if so return ship_id for the hit.
    pub fn handle_shot(&self, row: usize, col: usize) -> Option<usize>{
        for ship in &self.ships {
            if ship.point_in_ship(row, col) {       // Hit
                return Some(ship.ship_id);
            }
        }
        None                                // Miss
    }

// Remove the first ship
    pub fn remove_first_ship(&mut self) -> Option<ShipBoundingBox> {
        if self.ships.is_empty() {
            return None;
        }
        Some(self.ships.remove(0))
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
        if small <= 1 {
            return Err("Error: Smallest ship size must be greater than 1");
        }
    
        let large_val = large.unwrap_or(self.largestship.max(small));
    
        if large_val <= 1 || large_val < small {
            return Err("Error: Largest ship size must be at least as large as the smallest ship size and greater than 1");
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

    pub fn remove_first_board(&mut self) -> Option<PlayBoard> {
        if self.boards.is_empty() {
            return None;
        }
        Some(self.boards.remove(0))
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
        let mut tmp_end: (usize, usize) = (0,0);
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


    pub fn overlap_possible(
        &self, 
        ship2_size: usize, 
        ship2_point: (usize, usize), 
        ship2_dir: &Direction
    ) -> bool {
        let self_horizontal = self.start.1 == self.end.1;
        let ship2_horizontal = *ship2_dir == Direction::Horizontal;
    
        if self_horizontal && ship2_horizontal {
            // Both ships are horizontal → check if they are in the same row and their columns overlap
            return self.start.1 == ship2_point.1 // Same row
                && self.start.0 <= ship2_point.0 + ship2_size - 1 
                && ship2_point.0 <= self.end.0;
        }
    
        if !self_horizontal && !ship2_horizontal {
            // Both ships are vertical → check if they are in the same column and their rows overlap
            return self.start.0 == ship2_point.0 // Same column
                && self.start.1 <= ship2_point.1 + ship2_size - 1
                && ship2_point.1 <= self.end.1;
        }
    
        // At this point, one ship is vertical, and one is horizontal
        let (horiz_start, horiz_len, vert_start, vert_len) = if self_horizontal {
            (self.start, self.end.0 - self.start.0 + 1, ship2_point, ship2_size)
        } else {
            (ship2_point, ship2_size, self.start, self.end.1 - self.start.1 + 1)
        };
    
        // The horizontal ship must pass through the vertical ship's column
        // AND the vertical ship must pass through the horizontal ship's row
        horiz_start.0 <= vert_start.0 && vert_start.0 <= horiz_start.0 + horiz_len &&
        vert_start.1 <= horiz_start.1 && horiz_start.1 <= vert_start.1 + vert_len - 1
    }
    
    // Check for a collision between ships
    pub fn overlaps(&self, ship2: &ShipBoundingBox) -> bool {
        let self_horizontal = self.start.1 == self.end.1;
        let ship2_horizontal = ship2.start.1 == ship2.end.1;

        if self_horizontal && ship2_horizontal {
            // Both ships are horizontal → check if they are in the same row and their columns overlap
            return self.start.1 == ship2.start.1 // Same row
                && self.start.0 <= ship2.end.0 
                && ship2.start.0 <= self.end.0; // Overlapping column range
        }

        if !self_horizontal && !ship2_horizontal {
            // Both ships are vertical → check if they are in the same column and their rows overlap
            return self.start.0 == ship2.start.0 // Same column
                && self.start.1 <= ship2.end.1 
                && ship2.start.1 <= self.end.1; // Overlapping row range
        }

        // At this point, one ship is vertical, and one is horizontal
        let (horiz_ship, vert_ship) = if self_horizontal {
            (self, ship2)
        } else {
            (ship2, self)
        };

        // The horizontal ship must pass through the vertical ship's column
        // AND the vertical ship must pass through the horizontal ship's row
        horiz_ship.start.0 <= vert_ship.start.0 && vert_ship.start.0 <= horiz_ship.end.0 &&
        vert_ship.start.1 <= horiz_ship.start.1 && horiz_ship.start.1 <= vert_ship.end.1
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

// Return an option for 2d matrix with 0's for water and nums for ships
pub fn create_my_board_from_player(myboard: &GameData, player: &mut PlayBoard) -> Vec<Vec<usize>> {
    let (my_cols, my_rows) = myboard.get_col_row();
    let mut tmpboard = vec![vec![0; my_cols]; my_rows];  // Create 0-initialized board

    while let Some(this_ship) = player.remove_first_ship() {
        let (x1, y1) = this_ship.start;
        let (x2, y2) = this_ship.end;

        if y1 == y2 {  // Ship is horizontal
            for x in x1..=x2 {
                tmpboard[y1][x] = this_ship.ship_id;
            }
        } else {  // Ship is vertical
            for y in y1..=y2 {
                tmpboard[y][x1] = this_ship.ship_id;
            }
        }
    }

    tmpboard
}

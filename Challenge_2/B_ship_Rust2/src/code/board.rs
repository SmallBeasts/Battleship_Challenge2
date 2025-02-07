use std::vec;
use crate::code::enums::Direction;

// This structure will be the main board per player
pub struct PlayBoard {
    playername: String,
    playernum: i16,
    ships: Vec<BoundingBox>,                    // This is used in create to store only ships
    mine: Vec<Vec<i16>>
}

pub fn create_player(rows: i16, cols: i16) -> PlayBoard {
    let mut mine = Vec::with_capacity(rows as usize);

    for _ in 0..rows {
        mine.push(vec![0; cols as usize]);
    }

    PlayBoard {
        playername: String::new(),
        playernum: 0,
        ships: Vec::new(),
        mine: mine
    }
}

impl PlayBoard {
    pub fn get_playername(&self) -> String {
        self.playername.clone()
    }

    pub fn set_playername(&mut self, name: String) {
        self.playername = name;
    }

    pub fn get_playernum(&self) -> i16 {
        self.playernum
    }

    pub fn set_playernum(&mut self, num: i16) {
        self.playernum = num;
    }

    // TODO set ships to Vec
    // TODO get and set functions for board
}


// This will be the main game data storage.  Boards will only be stored inside a Vector
pub struct GameData {
    rows: i16,
    cols: i16,
    player_count: i16,
    loaded: bool,
    interactive: bool,
    filename: String,
    smallestship: i16,
    largestship: i16,
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

    pub fn get_shipsizes(&self) -> (i16, i16) {
        (self.smallestship, self.largestship)
    }

    pub fn set_shipsizes(&mut self, small: i16, large: Option<i16>) -> Result<(), &str> {
        let large_val = large.unwrap_or(small + 5);
        if small <= 1 || large_val <= 1 {
            return Err("Error: Ship sizes have to be positive and larger than 1");
        }
        self.smallestship = small;
        self.largestship = large_val;
        Ok(())
    }

    pub fn get_row_col(&self) -> (i16,i16) {
        (self.rows, self.cols)
    }
    // Set both rows and columns together if possible
    pub fn set_row_col(&mut self, row: i16, col: i16) {
        self.rows = row;
        self.cols = col;
    }
    
    // True means set row, false is cols
    pub fn set_row_or_col(&mut self, num: i16, row_col: bool) {
        if row_col {
            self.rows = num;
        } else {
            self.cols = num;
        }
    }

    pub fn get_playercount(&self) -> i16 {
        self.player_count
    }

    pub fn set_playercount(&mut self, num: i16) {
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

    //TODO fix the boards
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

// Definition of ships and their locations
impl BoundingBox {
    pub struct BoundingBox {
        pub ship_id: i16,
        pub start: (i16, i16),
        pub end: (i16, i16),
    }

    pub fn new(
        ship_id: i16,
        start: (i16, i16),
        length: i16,
        direction: Direction,
        board_width: i16,
        board_height: i16,
    ) -> Result<Self, String> {
        let end = match direction {
            Direction::Horizontal => (start.0 + length - 1, start.1),
            Direction::Vertical => (start.0, start.1 + length - 1),
        };

        // Validate that both start and end points are within bounds
        if start.0 < 0 || start.0 >= board_width || start.1 < 0 || start.1 >= board_height {
            return Err(format!("Error: Start position {:?} is out of bounds", start));
        }
        if end.0 < 0 || end.0 >= board_width || end.1 < 0 || end.1 >= board_height {
            return Err(format!("Error: End position {:?} is out of bounds", end));
        }

        Ok(Self { ship_id, start, end })
    }
    
    
    pub fn collision(&self, other: &BoundingBox) -> bool {
        let x_overlap = self.start.0 <= other.end.0 && self.end.0 >= other.start.0;
        let y_overlap = self.start.1 <= other.end.1 && self.end.1 >= other.start.1;

        x_overlap && y_overlap
    }
}
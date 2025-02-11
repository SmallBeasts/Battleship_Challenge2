pub struct ShipBoundingBox {
    pub ship_id: usize,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

pub struct GameBoard {
    pub width: usize,
    pub height: usize,
    // ... other game board data (e.g., grid representation) ...
}

impl ShipBoundingBox {
    // Constructor with validation
    pub fn new(ship_id: usize, start: (usize, usize), end: (usize, usize), board: &GameBoard) -> Option<ShipBoundingBox> {
        if !board.is_valid_coordinate(start) || !board.is_valid_coordinate(end) {
            return None; // Invalid coordinates
        }

        // Check for valid ship placement (horizontal or vertical)
        if start.0 != end.0 && start.1 != end.1 {
            return None; // Must be horizontal or vertical
        }

        Some(ShipBoundingBox { ship_id, start, end })
    }

    // Example method to check for overlap with another bounding box
    pub fn overlaps(&self, other: &ShipBoundingBox) -> bool {
        // ... logic to check for overlap ... (see below)
        // This is a simplified example. You might need more robust overlap detection
        // based on your game's rules (e.g., adjacent ships allowed or not).
        (self.start.0 <= other.end.0 && self.end.0 >= other.start.0) &&
        (self.start.1 <= other.end.1 && self.end.1 >= other.start.1)
    }
}


impl GameBoard {
    pub fn new(width: usize, height: usize) -> GameBoard {
        GameBoard { width, height }
    }

    pub fn is_valid_coordinate(&self, coord: (usize, usize)) -> bool {
        coord.0 < self.width && coord.1 < self.height
    }

    // Example of adding a ship (with collision check)
    pub fn add_ship(&mut self, ship: ShipBoundingBox, ships: &Vec<ShipBoundingBox>) -> Result<(), &'static str> {
        for existing_ship in ships {
            if ship.overlaps(existing_ship) {
                return Err("Ship overlaps with another ship.");
            }
        }

        // ... Add the ship to the game board's data structure ...
        Ok(())
    }
}

fn main() {
    let board = GameBoard::new(10, 10);
    let mut ships: Vec<ShipBoundingBox> = Vec::new();

    // Valid placement
    if let Some(ship1) = ShipBoundingBox::new(1, (1, 1), (1, 4), &board) {
        if let Ok(()) = board.add_ship(ship1, &ships){
            ships.push(ship1);
            println!("Ship 1 added");
        }
    }

    // Invalid placement (out of bounds)
    if ShipBoundingBox::new(2, (9, 9), (10, 10), &board).is_none() {
        println!("Ship 2 placement invalid");
    }

    // Invalid placement (diagonal)
    if ShipBoundingBox::new(3, (2, 2), (3, 3), &board).is_none() {
        println!("Ship 3 placement invalid");
    }

    // Overlapping placement
    if let Some(ship4) = ShipBoundingBox::new(4, (1, 2), (1, 5), &board) {
        if let Err(msg) = board.add_ship(ship4, &ships){
            println!("{}", msg);
        }
    }
}

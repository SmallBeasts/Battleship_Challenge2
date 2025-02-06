struct BattleShip {
    name: i16,

}


//ChatGPT code:
pub struct ShipBoundingBox {
    pub ship_id: i16,          // Identifier for the ship
    pub start: (i16, i16),     // Starting coordinate (col, row)
    pub end: (i16, i16),       // Ending coordinate (col, row)
}

impl ShipBoundingBox {
    #[derive(Debug)]
pub struct BoundingBox {
    pub ship_id: i16,
    pub start: (i16, i16),
    pub end: (i16, i16),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl BoundingBox {
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

    pub fn overlaps(&self, other: &BoundingBox) -> bool {
        let self_min_x = self.start.0.min(self.end.0);
        let self_max_x = self.start.0.max(self.end.0);
        let self_min_y = self.start.1.min(self.end.1);
        let self_max_y = self.start.1.max(self.end.1);

        let other_min_x = other.start.0.min(other.end.0);
        let other_max_x = other.start.0.max(other.end.0);
        let other_min_y = other.start.1.min(other.end.1);
        let other_max_y = other.start.1.max(other.end.1);

        // Check if the bounding boxes intersect
        !(self_max_x < other_min_x || self_min_x > other_max_x || self_max_y < other_min_y || self_min_y > other_max_y)
    }
}
}
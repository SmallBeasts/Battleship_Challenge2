// Error output for parse to int from string
pub enum RowColErr {
    Failed,
    TooSmall,
}

// Specific enum to give individual instances as errors.
pub enum QueryError {
    InvalidFormat,
    InvalidRow,
    InvalidColumn,
    OutOfBounds,
}

// Enum to declare state of create
#[derive(PartialEq)]
pub enum StateCreate {
    StateRows,
    StateCols,
    StateShips,
    StatePlayer,
    StateRandom,
    StatePlaceShip,
    StateFileName,
    StateCreate,
}

pub enum Direction {
    Horizontal,
    Vertical,
}

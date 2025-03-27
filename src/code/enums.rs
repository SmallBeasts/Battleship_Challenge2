use::std;

// Error output for parse to int from string
pub const MAX_SIZE: usize = std::i16::MAX as usize;
pub const MIN_SIZE: usize = std::i16::MIN as usize;

pub enum RowColErr {
    Failed,
    TooSmall,
    TooBig,
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
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Clone, Copy, Debug)]

pub enum HitMiss {
    Hit,
    Miss,
}

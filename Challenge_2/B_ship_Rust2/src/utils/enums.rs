// Error output for parse to int from string
enum RowColErr {
    Failed,
    TooSmall,
}

// Specific enum to give individual instances as errors.
enum QueryError {
    InvalidFormat,
    InvalidRow,
    InvalidColumn,
    OutOfBounds,
}

// Enum to declare state of create
enum StateCreate {
    StateRows,
    StateCols,
    StateShips,
    StatePlayer,
    StateRandom,
    StatePlaceShip,
    StateFileName,
    StateCreate,
}
use std::vec;

// This structure will be the main board per player
struct PlayBoard {
    playername: String,
    playernum: i16,
    ships: Vec<i16>,                                // This is used in create and in verify not load
    mine: Vec<Vec<i16>>
}

fn create_player(rows: i16, cols: i16) -> PlayBoard {
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

// This will be the main game data storage.  Boards will only be stored inside a Vector
struct GameData {
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

// This will create a new game board with empty Vec for boards
fn create_game() -> GameData {
    GameData {
        rows: 0,
        cols: 0,
        player_count: 0,
        loaded: false,
        interactive: false,
        filename: "".to_string(),
        smallestship: 2,
        largestship: 5,
        boards: Vec::new()
    }
}
use std::collections::btree_map::Values;
use std::fs::File;
use std::io::{self, BufRead, stdin, BufReader};
use std::io::Write;
use std::ptr::null;
use std::ptr::null_mut;
use std::vec;
use std::fmt;
use std::process;

mod game;   // Import the board and functions related to structures
mod utils;


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


fn output_string(buf: &str) {
    print!("{}\n:> ", buf);
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

// Handling repetitive file errors
fn handle_file_error(err: io::Error) -> bool {
    output_string(&format!("Error: Failed to open specified file: {}", err));
    false
}

// Handle repetitive parsing errors when converting string to int
fn handle_parse_error(err: &str) -> bool {
    output_string(&format!("Error: Failed to parse value: {}", err));
    false
}

fn handle_player_parse_error(err: &str) -> bool {
    output_string(&format!("Error: Failed to load player correctly: {}", err));
    false
}

fn load_file_game_data(line: &str, myboard: &mut GameData, line_num: i16) -> Result<(), String> {
    let mut tmp_line = line;
    if line.is_empty() {
        return Err(format!("Error: Empty line at {}", line_num));
    }
    // Rows handling
    match line_num {
        0 => {
            match tmp_line.trim().parse::<i16>() {
                Ok(num) => {
                    if num > 0 {
                        myboard.rows = num;
                        return Ok(());
                    }
                    else {
                        return Err(format!("Error: Invalid row_size value {}", num));
                    }
                }
                Err(err) => {
                    return Err(format!("Error: Failed to load player correctly: {}", err));
                }
            }
        }
        // Columns parsing
        1 => {
            match tmp_line.trim().parse::<i16>() {
                Ok(num) => {
                    if num > 0 {
                        myboard.cols = num;
                        return Ok(());
                    }
                    else {
                        return Err(format!("Error: Invalid col_size value {}", num));
                    }
                }
                Err(err) => {
                    return Err(format!("Error: Failed to load player correctly: {}", err));
                }
            }
        }
        // Handle player count
        2 => {
            match tmp_line.trim().parse::<i16>() {
                Ok(num) => {
                    if num < i16::MAX && num > 0 {
                        myboard.player_count = num;
                        return Ok(());
                    }
                    else {
                        return Err(format!("Error: Invalid player_count value {}", num));
                    }
                }
                Err(err) => {
                    return Err(format!("Error: Failed to load player correctly: {}", err));
                }
            }
        }
        _ => {
            return Err("Went too far, not sure why.".to_string())
        }
    }
}

// Pass the player data in as a whole, so iterate through.
fn load_player_game_data(lines: &mut impl Iterator<Item = io::Result<String>>, myboard: &mut GameData) -> Result<(), String> {

    let mut count = 0;
    let mut play_count = 0;
    while let Some(Ok(line)) = lines.next() {
        let data = line.trim();                 
        if data.contains(',') {                 // Board data outside of a player struct
            return Err(format!("Error: Unexpected data at line {}", count));
        }
        else {                                  // This should be player name
            count = 0;                          // Reset count
            if data.is_empty() {
                return Err(format!("Error: Playername is incorrect {}", play_count + 1));
            }
            else {
                play_count += 1;
                let mut newboard = create_player(myboard.rows, myboard.cols);
                newboard.playername = data.to_string();
                newboard.playernum = play_count;
                while let Some(Ok(row)) = lines.next() {              // Advance the line
                    count += 1;
                    if count > myboard.rows {
                        return Err(format!("Error: Too many rows in player {}", play_count));
                    }
                    let parts: Vec<&str> = row.as_str().split(',').collect();
                    if parts.len() > myboard.cols as usize {
                        return Err(format!("Error: Too many columns at row {}, in player {}", count, play_count));
                    }
                    for (j, num) in parts.iter().enumerate() {
                        match num.trim().parse::<i16>() {
                            Ok(val) => {
                                if count - 1 < newboard.mine.len() as i16 && j < newboard.mine[(count - 1) as usize].len() {
                                    newboard.mine[(count - 1) as usize][j] = val;
                                }
                                else {
                                    return Err(format!("Error: OOB at column {}, on row {}", j, count));
                                }
                            }
                            Err(err) => {
                                return Err(format!("Error: Failed to parse column at index {}", err));
                            }
                        }
                    }
                    if count == myboard.rows {
                        myboard.boards.push(newboard);
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

fn load_file(filename: &str, myboard: &mut GameData) ->bool {
    if filename.is_empty() {             // Empty string for filename
        return false                           // Return false
    }
    match File::open(filename) {
        Err(err) => handle_file_error(err), 
        Ok(file) => {                   // File is now open, time to read
            let reader = BufReader::new(file);
            let mut lines = reader.lines();
            for (line_num, line) in lines.by_ref().enumerate() {
                if line_num < 3{
                    match load_file_game_data(&line.unwrap(), myboard, line_num as i16) {
                        Ok(()) => continue,
                        Err(err) => {
                            output_string(&err);
                            return false
                        }
                    }
                }
                // Now we are into player names and the grids
                else {
                    break;
                }
            }
            // Now since we know the size of the boards and the players for loop through each line
            match load_player_game_data(&mut lines, myboard) {
                Ok(_) => {
                    myboard.loaded = true;
                    return true;
                }
                Err(err) => {
                    output_string(&format!("{}",err));
                    return false;
                }
            }
            
        }
    }
}

fn query_array(mybuf: &str, myboard: &mut GameData, playerid: i16) -> Result<i16, String> {
    let mut row_str = String::new();
    let mut col_str = String::new();
    if myboard.loaded == false {
        return Err("Error: You have not loaded a file yet!".to_string());
    }

    for c in mybuf.chars() {
        if c.is_ascii_uppercase() {
            row_str.push(c);
        } else if c.is_ascii_digit() {
            col_str.push(c);
        } else if !c.is_whitespace() {
            return Err("Invalid format in query.".to_string());
        }
    }
    if row_str.is_empty() || col_str.is_empty() {
        return Err(format!("Invalid query (missing row or column): {}", mybuf));
    }

    let mut row_index: usize = 0;
    for c in row_str.chars() {
        row_index = row_index * 26 + (c as u8 - b'A' + 1) as usize;
    }
    row_index -= 1;                         // A is row 0

    let col_index = match col_str.parse::<usize>() {
        Ok(n) => n - 1,
        Err(_) => {
            return Err("Invalid column number".to_string());
        }
    };

    if row_index < myboard.boards[playerid as usize].mine.len() && col_index < myboard.boards[playerid as usize].mine[row_index].len() {
        Ok(myboard.boards[playerid as usize].mine[row_index][col_index])
    }
    else {
        Err(format!("Query out of bounds: Row {}, Column {}", row_index + 1, col_index + 1))
    }
}


fn parse_command(mybuf: &str, myboard: &mut GameData) -> bool {
    let upper = mybuf.to_uppercase();
    let mut tokens = upper.split_whitespace();
    let command = tokens.next();
    match command {                                         // Tokenize and split the command
        Some("LOAD") => {
            if myboard.loaded {
                output_string("A previous board was loaded, now loading new file.");
                *myboard = create_game();
            }
            if let Some(filename) = tokens.next() {
                output_string(&format!("Loading the file <{}>", filename));
                if load_file(filename, myboard) {
                    output_string("File loaded successfully");
                } else {
                    output_string("File failed to load");
                }
            } else {
                output_string("Usage: --load <filename>");
            }
            false
        }
        Some("GUESS") => {
            if myboard.loaded == false {        // Not initialized with a load
                output_string("You have not loaded a file yet.");
                return false;
            }
            let mut results = Vec::new();
            let mut oob_messages = Vec::new();

            for tok in tokens {
                let mut tmp_id: i16 = -1;
                let guesses: Vec<&str> = tok.split(',').collect();
                for (guess_num, guess) in guesses.iter().enumerate() {
                    if guess_num == 0 {
                        match guess.parse::<i16>() {
                            Ok(n) => {
                                if n > myboard.player_count || n < 1 {
                                    output_string(&format!("Error: Invalid playerid, {}", n));
                                    return false;
                                }
                                else {
                                    tmp_id = n;
                                }
                            }
                            Err(_) => {
                                output_string(&format!("Error: Invalid playerid, {}", guess));
                                return false;
                            }
                        }
                    }
                    match query_array(guess, myboard, tmp_id) { // Call query_array directly
                        Ok(value) => results.push(value.to_string()),
                        Err(msg) => {
                            results.push("OOB".to_string());
                            oob_messages.push(msg);
                        }
                    }
                }
            }

            if !oob_messages.is_empty() {
                for msg in oob_messages {
                    output_string(&msg);
                }
            }
            output_string(&format!("Results are {}", results.join(",")));
            return false;
        },
        Some("HELP") => {
            output_string("Available commands: --load <filename>\n--guess <list in A1 or AA10 format>\n--help this output\n--exit or --quit to quit.");
            false
        }
        Some("EXIT") | Some("QUIT") => {
            output_string("Thank you for enjoying Battleship Test in Rust!");
            true
        }
        None => {
            output_string("No command was recognized type --help for a list of commands");
            return false
        }
        _ => {                  // Handle only commands that are not known commands 0 is default player
            match query_array(upper.as_str(), myboard, 0) {
                Ok(value) => {
                    output_string(&format!("{}", value));
                    return false;
                },
                Err(msg) => {
                    output_string(&msg);
                    return false;
                }
            }
        }
    }
}
    
        
fn eval_input(mybuf: String, myboard: &mut GameData) -> bool {
    if mybuf.contains("--") {
        let tokens = mybuf.split("--").filter(|&x| !x.is_empty());     // Split on token command
        for tok in tokens {
            if parse_command(tok, myboard) {
                return true;
            }
        }
    } else {
        if parse_command(mybuf.as_str(), myboard) {
            return true;
        }
    }
    false
}

// Function to return base 26 for the columns
fn base_26(buf: String) -> i16 {
    let mut col_index: i16 = 0;
    for c in buf.chars() {
        col_index = col_index * 26 + (c as u8 - b'A') as i16;
    }
    col_index
}

// Function to translate a Column Row notation into a query notation
fn translate_query(mybuf: &str) -> Result<(i16, i16), QueryError> {
    let mut row_raw = String::new();
    let mut col_raw = String::new();

    for c in mybuf.to_uppercase().chars() {
        if c.is_ascii_uppercase() {
            col_raw.push(c);
        } else if c.is_ascii_digit() {
            row_raw.push(c);
        } else if c.is_whitespace() {
            continue;
        }
    }

    if row_raw.is_empty() || col_raw.is_empty() {
        return Err(QueryError::InvalidFormat);
    }

    let mut col_index = base_26(col_raw);

    let row_index = row_raw.parse::<i16>().map_err(|_| QueryError::InvalidRow)?;

    Ok((col_index, row_index))
}

enum RowColErr {
    Failed,
    TooSmall,
}

fn create_row_col(RowCol: &str) -> Result<i16, RowColErr> {
    match RowCol.parse::<i16>() {
        Ok(n) => {
            if n <= 0 {
                return Err(RowColErr::TooSmall);
            }
            else {
                Ok(n)
            }
        },
        Err(_) => Err(RowColErr::Failed),
    }
}

// This function only is for the command line
fn command_line_input(myboard: &mut GameData) {
    let args: Vec<String> = std::env::args().collect();
    let mut args_iter = args.iter().skip(1); // Skip program name
    let mut mystate: Vec<StateCreate>;      // Keep track of the state of Create commands

    while let Some(arg) = args_iter.next() {
        match arg.to_uppercase().as_str() {
            "--LOAD" => {
                if myboard.loaded {
                    *myboard = create_game();
                }
                if let Some(filename) = args_iter.next() {
                    if load_file(filename, myboard) {
                        output_string("File loaded successfully.");
                    } else {
                        output_string("File was not found, please enter the full path.");
                    }
                } else {
                    output_string("Usage: --load <filename>");
                }
            }
            "--HELP" => {
                output_string(
                    "Available commands: \
                     --load <filename>\n--guess <list in A1 or AA10 format>\n--help (this output)\n--exit or --quit to quit.",
                );
            }
            "--EXIT" | "--QUIT" => {
                output_string("Thank you for enjoying Battleship Test Rust version 1.");
                break;
            }
            "--GUESS" => {
                let mut guesses = String::new();
                let mut count = 0;

                // Process guesses until we encounter a new command or run out of arguments
                let mut tmp_id = -1;        // Initialize to invalid playerid
                while let Some(next_guess) = args_iter.next() {
                    if next_guess.starts_with("--") {
                        break; // Stop processing guesses
                    }
                    if count == 0 {
                        match next_guess.parse::<i16>() {
                            Ok(n) => {
                                if n < 1 || n > myboard.player_count {
                                    output_string(&format!("Error: Playerid is invalid {}", n));
                                    return false;
                                }
                                else {
                                    tmp_id = n;
                                    count += 1;
                                    continue;
                                }
                            },
                            Err(_) => {
                                output_string(&format!("Error: Invalid playerid {}", next_guess.as_str()));
                                break;
                            }
                        };
                    }
                    match translate_query(next_guess) {
                        Ok((col, row)) => {
                            if col >= myboard.cols as i16 || row >= myboard.rows as i16 {
                                guesses.push_str("OOB,"); // Out of bounds
                            } else {
                                guesses.push_str(&format!("{},", myboard.boards[tmp_id as usize].mine[row as usize][col as usize]));
                            }
                        }
                        Err(err) => {
                            match err {
                                QueryError::InvalidFormat => output_string("Invalid guess format"),
                                QueryError::InvalidRow => output_string("Invalid row format"),
                                QueryError::InvalidColumn => output_string("Invalid column format"),
                                QueryError::OutOfBounds => output_string("Guess is out of bounds"),
                            }
                        }
                    }
                }

                if !guesses.is_empty() {
                    guesses.pop(); // Remove trailing comma
                }
                output_string(&guesses);
            }
            Some("--VERIFY") => {
                // Function call for Verify with path
            },
            Some("--CREATE") => {
                // Function call for Create with path
                let mut myboard = create_game();            // Create a new board to start population
                
                if let Some(next_guess) = args_iter.next() {
                    myboard.filename = next_guess;
                    mystate.push(StateCreate::StateFileName);   // Keep track that a filename was added
                    mystate.push(StateCreate::StateCreate);     // Keep track that we are in create
                }
                else {
                    output_string("Error: Missing path for Create command");
                    return false;
                }
            },
            Some("--ROW") => {
                if !mystate.contains(StateCreate::StateCreate) {
                    output_string("Error: Called Row without a valid create and file path");
                    return false;
                }
                if let Some(next_guess) = args_iter.next() {
                    match create_row_col(next_guess) {
                        Ok(row) => {
                            mystate.push(StateCreate::StateRows);
                            myboard.rows = row;
                        }
                        Err(err) => {
                            match err {
                                RowColErr::Failed => output_string("Error: Rows value is not a valid integer"),
                                RowColErr::TooSmall => output_string("Error: Rows value can not be 0 or negative"),
                            }
                            return false;
                        }
                    }
                }
                else {
                    output_string("Error: Missing rows value after row in create");
                    return false;
                }
            },
            Some("--COL") => {
                if !mystate.contains(StateCreate::StateCreate) {
                    output_string("Error: Called Col without a valid create and file path");
                    return false;
                }
                if let Some(next_guess) = args_iter.next() {
                    match create_row_col(next_guess) {
                        Ok(col) => {
                            mystate.push(StateCreate::StateCols);
                            myboard.cols = col;
                        }
                        Err(err) => {
                            match err {
                                RowColErr::Failed => output_string("Error: Columns value is not a valid integer"),
                                RowColErr::TooSmall => output_string("Error: Columns value can not be 0 or negative"),
                            }
                            return false;
                        }
                    }
                }
                else {                                          // Empty value after columns
                    output_string("Error: Missings columns value after col in create");
                    return false;
                }
            },             
            Some("--SHIPS") => {
                if !mystate.contains(StateCreate::StateCreate) {
                    output_string("Error: Called Ships without a valid create and file path");
                    return false;
                }
                if let Some(next_guess) = args_iter.next() {
                    match create_row_col(next_guess) {
                        Ok(n) => {
                            if mystate.iter().any(|&x| x== StateCreate::StateShips) {       // Ships has been called before
                                if myboard.smallestship > n {                               // Check that the old call was less than the new
                                    myboard.largestship = myboard.smallestship;
                                    myboard.smallestship = n;
                                }
                                else {
                                    myboard.largestship = n;
                                }
                            }
                            else {
                                myboard.smallestship = n;
                                mystate.push(StateCreate::StateShips);
                            }
                        },
                        Err(err) => {
                            match err {
                                RowColErr::Failed => output_string("Error: Unable to convert Ships value to integer"),
                                RowColErr::TooSmall => output_string("Error: A ship must be at least 1 or greater"),
                            }
                            return false;
                        }
                    }
                }
            },
            Some("--PLAYER") => {                   // Add a new player name
                if !mystate.contains(StateCreate::StateCreate) {
                    output_string("Error: Called Player without a valid create or file path");
                    return false;
                }
                if !mystate.contains(StateCreate::StateRows) {          // Set rows to default value if not previously explicitly set
                    myboard.rows = 10;
                }
                if !mystate.contains(StateCreate::StateCols) {          // Set cols to default if not previously explicitly set
                    myboard.cols = 10;
                }
                if let Some(next_guess) = args_iter.next() {            // A string to name a player
                    if next.guess.starts_with("--") {
                        output_string(&format!("Error: Found command {} instead of a player name.", next_guess));
                        return false;
                    }
                    let tmp_player = create_player(myboard.rows, myboard.cols);
                    tmp_player.playername = next_guess;
                    tmp_player.playerid = myboard.boards.len() + 1;
                    if tmp_player.playerid > myboard.player_count {
                        output_string(&format!("Error: Too many players, declared {} players and created {} players", myboard.player_count, tmp_player.playerid));
                        return false;
                    }
                    mystate.push(StateCreate::StatePlayer);
                    myboard.boards.push(tmp_player);
                }
                else {
                    output_string("Error: Empty playername found!");
                    return false;
                }
            },
            Some("--RANDOM") => {
                if !mystate.contains(StateCreate::StateCreate) || !mystate.contains(StateCreate::StatePlayer) {
                    output_string("Error: Please declare a file path and a player name before using Random");
                    return false;
                }
                // Function to place ships
            },
            Some("--DISPLAY") => {
                // Function call to display the file with path
            },
            _ => {
                output_string(&format!("Unknown command: {}", arg));
            }
        }
    }
}


fn main(){
    output_string("Welcome to the Battleship Test Program v2\nYou can type --help to get a list of commands");
    let mut myboard = create_game();
    if std::env::args().len() <= 1 {
        output_string("No command line arguments entered.");
        myboard.interactive = true;
    } else {
        command_line_input(&mut myboard);
    }
    if myboard.interactive == true {                                // Only enter loop if interactive set
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            if eval_input(buffer, &mut myboard) {
                break;
            }
        }
    }
}

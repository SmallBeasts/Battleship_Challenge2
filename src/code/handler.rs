use crate::code::enums::StateCreate;
use crate::code::board::GameData;
use crate::code::utils::output_string;
use crate::code::enums::{RowColErr, Direction};
use crate::code::board::PlayBoard;
use crate::code::file;
use crate::code::board::ShipBoundingBox;
use rand::{Rng, rng};
use crate::code::utils;
use crate::code::enums;
use crate::code::enums::QueryError;
use crate::code::board;

// Function to handle loading files
pub fn handle_load(
    myboard: &mut GameData,
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>, 
    is_verify: bool) 
{
    // If there is already a board loaded reinitialize
    if myboard.get_loaded() {
        *myboard = GameData::default();
    }
    if let Some(filename) = args_iter.next() {
        match file::load_file(filename, myboard) {
            Ok(_) => {
                if is_verify {
                    output_string("Passed: File loaded successfully.");
                } else {
                    output_string("File loaded successfully.");
                }
            },
            Err(err) => {
                if is_verify {
                    output_string(&format!("Failed: {}", err));
                } else {
                     output_string(&format!("Error: Failed to load file: {}", err));
                }
            }
        }
    } else {
        output_string("Usage: --load <filename>");
    }
}

pub fn handle_row_col_error(err: RowColErr, is_col: bool) {
    let err_msg = match err {
        RowColErr::Failed => if is_col { "Error: Column is not a valid value" } else { "Error: Row is not a valid value" },
        RowColErr::TooSmall => if is_col { "Error: Column value must be greater than or equal to 1" } else { "Error: Row value must be greater than or equal to 1" },
        RowColErr::TooBig => {
            let max_size = enums::MAX_SIZE;  // Avoid referencing temporary format string
            return output_string(&format!("Error: {} value must be less than {}", if is_col { "Column" } else { "Row" }, max_size));
        }
    };
    output_string(&err_msg);
}

// Handle conversion and storage of row and column data in Create specifically but later probably in load.
pub fn handle_row_col(
    myboard: &mut GameData, 
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>, 
    is_col: bool 
) -> Result<(), RowColErr> {
    if let Some(next_value) = args_iter.next() {
        match utils::parse_to_usize(next_value) {
            Ok(value) => {
                myboard.set_row_or_col(value, is_col);
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else {
        Err(RowColErr::Failed)
    }
}


// Handle function to start the file creation
pub fn handle_create(myboard: &mut GameData, args_iter: &mut std::iter::Skip<std::slice::Iter<String>>,
                mystate: &mut Vec<StateCreate>) -> bool {
     // Function call for Create with path
     *myboard = GameData::default();            // Create a new board to start population
                
     if let Some(next_guess) = args_iter.next() {
         myboard.set_filename(next_guess.to_string());
         mystate.push(StateCreate::StateFileName);   // Keep track that a filename was added
         mystate.push(StateCreate::StateCreate);     // Keep track that we are in create
     }
     else {
         output_string("Error: Missing path for Create command");
         return false;
     }
     true
}

// Handle function to set ship min/max size
pub fn handle_ships_size(
    myboard: &mut GameData,
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>,
    mystate: &mut Vec<StateCreate>
    ) -> bool {

    if !mystate.contains(&StateCreate::StateCreate) {
        output_string("Error: Called Ships without a valid create and file path");
        return false;
    }
    if let Some(next_guess) = args_iter.next() {
        match utils::parse_to_usize(next_guess) {
            Ok(n) => {
                if mystate.contains(&StateCreate::StateShips) {
                    // Ships has been called before
                    let (_small, large) = myboard.get_shipsizes(); // large is already usize
                    let new_large = Some(large.max(n)); 
                    
                    myboard.set_shipsizes(n, new_large);
                }
                else {
                    myboard.set_shipsizes(n, None);
                    mystate.push(StateCreate::StateShips);
                }
            },
            Err(err) => {
                match err {
                    RowColErr::Failed => output_string("Error: Unable to convert Ships value to integer"),
                    RowColErr::TooSmall => output_string("Error: A ship must be at least 1 or greater"),
                    RowColErr::TooBig => output_string(&format!("Error: A ship must be smaller than {}", enums::MAX_SIZE)),
                }
                return false;
            }
        }
    }    
    true
}

// This is the first function call that requires everything else to be set
// Sets default rows/cols if not previously set

pub fn handle_player(
    myboard: &mut GameData,
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>,
    mystate: &mut Vec<StateCreate>) -> bool {
        
        
    if !mystate.contains(&StateCreate::StateCreate) {
        output_string("Error: Called Player without a valid create or file path");
        return false;
    }
    if let Some(next_guess) = args_iter.next() {            // A string to name a player
        if next_guess.starts_with("--") {
            output_string(&format!("Error: Found command {} instead of a player name.", next_guess));
            return false;
        }
        let mut tmp_player = PlayBoard::default();
        tmp_player.set_playername(next_guess.to_string());
        tmp_player.set_playernum(myboard.get_boards_len() + 1);
        mystate.push(StateCreate::StatePlayer);
        myboard.increment_playercount();
        myboard.boards_add(tmp_player);  // Add the player and update player count
    }
    else {
        output_string("Error: Empty playername found!");
        return false;
    }
    true
}

pub fn handle_help() {
    output_string(
        "Available commands: \
         --load <filename>\n--guess <list in A1 or AA10 format>\n--help (this output)\n--exit or --quit to quit.",
    );
}

use std::collections::HashMap;
use rand::seq::SliceRandom; // For choosing a random item

pub fn handle_random(myboard: &mut GameData, mystate: &mut Vec<StateCreate>) -> bool {
    if !mystate.contains(&StateCreate::StatePlayer) {
        output_string("Error: No player currently under creation.");
        return false;
    }

    let (small, large) = myboard.get_shipsizes();
    let (max_col, max_row) = myboard.get_col_row();

    if let Some(mut myplayer) = myboard.boards_pop_last() {
        let mut rng = rand::thread_rng(); // Initialize random number generator

        for ship_size in (small..=large).rev() {  // Place largest ships first
            let mut valid_positions: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();

            // Step 1: Find all valid ship placements
            for row in 0..=max_row {
                for col in 0..=max_col {
                    let mut directions = Vec::new();

                    // Check Horizontal Placement
                    if col + ship_size <= max_col + 1 && !myplayer.check_collision((col, row), ship_size, Direction::Horizontal) {
                        directions.push(Direction::Horizontal);
                    }
                    
                    // Check Vertical Placement
                    if row + ship_size <= max_row + 1 && !myplayer.check_collision((col, row), ship_size, Direction::Vertical) {
                        directions.push(Direction::Vertical);
                    }

                    if !directions.is_empty() {
                        valid_positions.insert((col, row), directions);
                    }
                }
            }

            // Step 2: If no valid placement, print a warning
            if valid_positions.is_empty() {
                output_string(&format!("Warning: No space for ship size {}", ship_size));
                continue;
            }

            // Step 3: Pick a random valid placement
            let valid_keys: Vec<&(usize, usize)> = valid_positions.keys().collect();
            if let Some(&(col, row)) = utils::pick_random(&valid_keys) {
                if let Some(directions) = valid_positions.get(&(*col, *row)) {
                    if let Some(&dir) = utils::pick_random(directions) {
                        // Step 4: Place the ship
                        if let Some(ship) = ShipBoundingBox::new(ship_size, (*col, *row), dir, myboard, &myplayer) {
                            output_string(&format!("Added ship {}, Col {}, Row {}", ship_size, col, row));
                            myplayer.add_ship(ship);
                        }
                    }
                }
            }
        }

        // Step 5: Add the updated player back to the board
        myboard.boards_add(myplayer);
        return true;
    }
    false
}


pub fn handle_guess(myboard: &mut GameData, 
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>) {
    output_string("Suck it!");
}

pub fn handle_display(myboard: &mut GameData,
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>) -> bool{
    if let Some(next_guess) = args_iter.next() {
        if next_guess.starts_with("--") {
            output_string("Error: expected filename and found command.");
            return false;
        }
        let filename = next_guess;                      // Use the filename given
    }
    else {
        let filename = myboard.get_filename();          // Use the filename in the given board
    }
    while let Some(mut player) = myboard.remove_first_board() {
        let tmpboard = board::create_my_board_from_player(myboard, &mut player);
        if let Some(largest) = player.get_largest_ship_id() {
            let mut x = 1;
            let mut count: usize = 0;
            while x < largest {                     // Get the number of digits for padding
                count += 1;
                x = x * 10;
            }
            utils::display_board(myboard, &tmpboard, count);
        } else {
            output_string("Error: No ships enrolled!");
            return false;
        }
    }
    true
}

pub fn handle_write_file(myboard: &mut GameData) {
    file::write_file(myboard);
}

pub fn handle_place_ship(myboard: &mut GameData, 
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>,
    mystate: &mut Vec<StateCreate>) -> bool{

    if !mystate.contains(&StateCreate::StateCreate) || !mystate.contains(&StateCreate::StatePlayer) {
        output_string("Error: Called Ships without a valid create and file path or without a player.");
        return false;
    }
    let mut ship_data = Vec::new();
    while let Some(next_guess) = args_iter.next() {
        if next_guess.starts_with("--") {
            output_string("Error: No status for the new ship.");
            return false;
        }
        if next_guess.contains(":") {
            ship_data.push(next_guess.clone());
        } else {
            output_string("Error: Invalid format for ship placement.");
            return false;
        }
    }
    process_ship_data(ship_data, myboard);
    true
}

fn process_ship_data(ship_data: Vec<String>, myboard: &mut GameData) -> bool {
    for ship in ship_data {
        let ship_coords: Vec<&str> = ship.split(':').collect();
        if ship_coords.len() != 3 {
            output_string("Error: Invalid ship format.");
            return false;
        }

        // Parse Ship ID
        let ship_id = match utils::parse_to_usize(ship_coords[0]) {
            Ok(ship_tmp) => {
                let (smallest, largest) = myboard.get_shipsizes();
                if ship_tmp >= smallest && ship_tmp <= largest {
                    ship_tmp
                } else {
                    output_string("Error: Ship size out of allowed range.");
                    return false;
                }
            }
            Err(_) => {
                output_string("Error: Invalid ship size.");
                return false;
            }
        };

        // Parse Coordinates
        let (my_col, my_row) = match utils::translate_query(ship_coords[1]) {
            Ok(coords) => coords,
            Err(err) => {
                match err {
                    QueryError::InvalidFormat => output_string("Error: Invalid coordinate format."),
                    QueryError::InvalidRow => output_string("Error: Invalid row"),
                    QueryError::InvalidColumn => output_string("Error: Invalid column"),
                    QueryError::OutOfBounds => output_string("Error: Out of bounds!!"),
                }
                return false;
            }
        };
        let coords = ship_coords[1].to_uppercase();
        let (mut col_str, mut row_str) = (String::new(), String::new());
        
        for c in coords.chars() {
            if c.is_ascii_uppercase() {
                col_str.push(c);
            } else if c.is_ascii_digit() {
                row_str.push(c);
            } else if !c.is_whitespace() {
                output_string("Error: Invalid ship entry format.");
                return false;
            }
        }

        if col_str.is_empty() || row_str.is_empty() {
            output_string("Error: Invalid ship entry format.");
            return false;
        }

        let my_row = match utils::parse_to_usize(&row_str) {
            Ok(n) => n,
            Err(_) => {
                output_string("Error: Invalid row number.");
                return false;
            }
        };

        let my_col = utils::base_26(col_str);

        // Parse Direction
        let up_ship = ship_coords[2].to_uppercase();
        let direction = match up_ship.as_str() {
            "V" => Direction::Vertical,
            "H" => Direction::Horizontal,
            _ => {
                output_string("Error: Invalid direction.");
                return false;
            }
        };

        // Retrieve Last Player
        let mut myplayer = match myboard.boards_pop_last() {
            Some(player) => player,
            None => {
                output_string("Error: No player available.");
                return false;
            }
        };

        // Create Ship
        if let Some(new_ship) = ShipBoundingBox::new(ship_id, (my_col, my_row), direction, myboard, &myplayer) {
            myplayer.add_ship(new_ship);
            myboard.boards_add(myplayer);
        } else {
            output_string("Error: Failed to create ship.");
            return false;
        }
    }
    
    true
}

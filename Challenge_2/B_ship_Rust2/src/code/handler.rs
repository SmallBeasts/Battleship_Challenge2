use crate::code::enums::StateCreate;
use crate::code::board::GameData;
use crate::code::utils::output_string;
use crate::code::enums::RowColErr;
use crate::code::file;
use crate::code::board::ShipBoundingBox;
use rand::{Rng, thread_rng};

// Function to handle loading files
pub fn handle_load(
    myboard: &mut GameData,
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>) 
{
    // If there is already a board loaded reinitialize
    if myboard.get_loaded() {
        *myboard = create_game();
    }
    if let Some(filename) = args_iter.next() {
        if file::load_file(filename, myboard) {
            output_string("File loaded successfully.");
        } else {
            output_string("File was not found, please enter the full path.");
        }
    } else {
        output_string("Usage: --load <filename>");
    }
}

pub fn handle_row_col_error(err: RowColErr, is_row: bool) {
    let err_msg = match err {
        RowColErr::Failed => {
            if is_row {
                "Error: Row is not a valid value"
            } else {
                "Error: Column is not a valid value"
            }
        }
        RowColErr::TooSmall => {
            if is_row {
                "Error: Row value must be greater than or equal to 1"
            } else {
                "Error: Column value must be greater than or equal to 1"
            }
        }
        RowColErr::TooBig => {
            if is_row {
                &format!("Error: Row value must be less than {}", MAX_SIZE)
            } else {
                &format!("Error: Column value must be less than {}", MAX_SIZE)
            }
        }
    };
    output_string(err_msg);
}

// Handle conversion and storage of row and column data in Create specifically but later probably in load.
pub fn handle_row_col(
    myboard: &mut GameData, 
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>, 
    row_col: bool
) -> Result<(), RowColErr> {
    if let Some(next_value) = args_iter.next() {
        match parse_to_usize(next_value) {
            Ok(value) => {
                if row_col {
                    myboard.set_row_or_col(value, true);
                } else {
                    myboard.set_row_or_col(value, false);
                }
                return Ok(());
            }
            Err(err) => return Err(err),
        }
    } else {
        return Err(RowColErr::Failed);
    }
}

// Handle function to start the file creation
pub fn handle_create(myboard: &mut GameData, args_iter: &mut std::iter::Skip<std::slice::Iter<String>>,
                mystate: &mut Vec<StateCreate>) -> bool {
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
        match code::utils::parse_to_usize(next_guess) {
            Ok(n) => {
                if mystate.contains(&StateCreate::StateShips) {       // Ships has been called before
                    let (small, large) = myboard.get_shipsizes();
                    if small > n {                               // Check that the old call was less than the new
                        myboard.set_shipsizes(n, Some(small));
                    }
                    else {
                        myboard.set_shipsizes(small, Some(n));
                    }
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
        let (row, col) = myboard.get_row_col();
        let tmp_player = code::board::create_player(row, col);
        tmp_player.set_playername(next_guess);
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

pub fn handle_random(myboard: &mut GameData, mystate: &mut Vec<StateCreate>) -> bool{
    if !mystate.contains(&StateCreate::StatePlayer) {
        output_string("Error: No player currently under creation.");
        return false;
    }
    let (small, large) = myboard.get_shipsizes();
    let (max_row, max_col) = myboard.get_row_col();
    let mut ship_count = small;
    let mut range = thread_rng();

    if let Some(mut myplayer) = myboard.boards_pop_last() {
        while ship_count <= large {
            let row = rng.gen_range(0..= max_row);
            let col = rng.gen_range(0..= max_col);
            let vert_horz = rng.gen_range(0..= 100);
            let dir = if vert_horz % 2 == 0 {
                Direction::Horizontal
            } else {
                Direction::Vertical
            };
            let my_new_ship = ShipBoundingBox::new(ship_count,  (row, col), dir, myboard, &myplayer);
            if let Some(ship) = my_new_ship {
                myplayer.add_ship(ship);
                ship_count += 1;
            } else {
                continue;
            }

        }
        myboard.boards_add(myplayer);
        return true;
    } else {
        return false;
    }
}
use std::fs::File;
use std::io::{self, BufRead, stdin, BufReader};
use std::io::Write;
use std::ptr::null;
use std::ptr::null_mut;
use std::vec;
use std::fmt;
use std::process;

use create::utils;
use crate::game;

fn handle_row_col_error(err: RowColErr, is_row: bool) {
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
    };
    output_string(err_msg);
}

// Handle conversion and storage of row and column data in Create specifically but later probably in load.
fn handle_row_col(
    myboard: &mut GameData, 
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>, 
    row_col: bool
) -> Result<(), RowColErr> {
    if let Some(next_value) = args_iter.next() {
        match create_row_col(next_value) {
            Ok(value) => {
                if row_col {
                    myboard.rows = value;
                } else {
                    myboard.cols = value;
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
fn handle_create(myboard: &mut GameData, args_iter: &mut std::iter::Skip<std::slice::Iter<String>>,
                mystate: &mut Vec<StateCreate>) {
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
}

// Handle function to set ship min/max size
fn handle_ships_size(
    myboard: &mut GameData,
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>,
    mystate: &mut Vec<StateCreate>
    ) {

    if !mystate.contains(StateCreate::StateCreate) {
        output_string("Error: Called Ships without a valid create and file path");
        return false;
    }
    if let Some(next_guess) = args_iter.next() {
        match parse_to_int(next_guess) {
            Ok(n) => {
                if mystate.contains(&StateCreate::StateShips) {       // Ships has been called before
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
}

// This is the first function call that requires everything else to be set
// Sets default rows/cols if not previously set

fn handle_player(
    myboard: &mut GameData,
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>,
    mystate: &mut Vec<StateCreate>) {
        
        
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
        myboard.player_count += 1;                          // Update the total number of players
        mystate.push(StateCreate::StatePlayer);
        myboard.boards.push(tmp_player);
    }
    else {
        output_string("Error: Empty playername found!");
        return false;
    }
}


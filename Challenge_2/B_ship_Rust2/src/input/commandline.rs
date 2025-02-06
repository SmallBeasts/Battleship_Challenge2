use std::fs::File;
use std::io::{self, BufRead, stdin, BufReader};
use std::io::Write;
use std::ptr::null;
use std::ptr::null_mut;
use std::vec;
use std::fmt;
use std::process;

// Refactored command line function
fn command_line_input(myboard: &mut GameData) {
    let args: Vec<String> std::env::args().collect();
    let mut args_iter = args.iter().skip(1);        // Skip program name
    let mut mystate: Vec<StateCreate>;              // Keep track of the state of Create commands

    while let Some(arg) = args_iter.next() {
        match arg.to_uppercase().as_str() {
            "--LOAD" => handle_load(myboard, &mut args_iter),
            "--HELP" => handle_help(),
            "--EXIT" || "--QUIT" => {
                output_string("Thank you for playing!");
                break;
            }
            "--GUESS" => handle_guess(myboard, &mut args_iter),
            "--VERIFY" => handle_verify(myboard, &mut args_iter),
            "--CREATE" => handle_create(myboard, &mut args_iter, &mut mystate),
            "--ROW" => {
                if let Err(err) = handle_row_col(myboard, &mut args_iter, true) {
                    handle_row_col_error(err, true);
                    return false;
                }
            },
            "--COL" => {
                if let Err(err) = handle_row_col(myboard, &mut args_iter, false) {
                    handle_row_col_error(err, false);
                    return false;
                }
            },
            "--SHIPS" => handle_ships_size(myboard, &mut args_iter),
            "--PLAYER" => handle_player(myboard, &mut args_iter),
            "--RANDOM" => handle_random(myboard, &mut args_iter),
            "--DISPLAY" => handle_display(myboard, &mut args_iter),
        }
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

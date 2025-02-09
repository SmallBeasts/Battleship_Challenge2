use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::code::board::GameData;
use crate::code::utils::output_string;
use crate::code::utils::parse_to_usize;


fn load_file_game_data(line: &str, myboard: &mut GameData, line_num: i16) -> Result<(), String> {
    let tmp_line = line;
    if line.is_empty() {
        return Err(format!("Error: Empty line at {}", line_num));
    }
    // Rows handling
    match line_num {
        0 => {
            match parse_to_usize(tmp_line.trim()) {
                Ok(num) => {
                    myboard.set_row_or_col(num, true);
                    return Ok(());
                }
                Err(err) => {
                    return Err(format!("Error: Failed to load player correctly: {}", err));
                }
            }
        }
        // Columns parsing
        1 => {
            match parse_to_usize(tmp_line.trim()) {
                Ok(num) => {
                    myboard.set_row_or_col(num, false);
                    return Ok(());
                }
                Err(err) => {
                    return Err(format!("Error: Failed to load player correctly: {}", err));
                }
            }
        }
        // Handle player count
        2 => {
            match parse_to_usize(tmp_line.trim()) {
                Ok(num) => {
                    myboard.set_playercount(num);
                    return Ok(());
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
                let (myrow, mycol) = myboard.get_row_col();
                let mut newboard = board::create_player(myrow, mycol);
                newboard.set_playername(data.to_string());
                newboard.set_playernum(play_count);
                while let Some(Ok(row)) = lines.next() {              // Advance the line
                    count += 1;
                    if count > myrow {
                        return Err(format!("Error: Too many rows in player {}", play_count));
                    }
                    let parts: Vec<&str> = row.as_str().split(',').collect();
                    if parts.len() > mycol as usize {
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
                    if count == myrow {
                        myboard.boards_add(newboard);
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
        Err(err) => utils::handle_file_error(err), 
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
                    myboard.set_loaded(true);
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
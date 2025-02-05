use std::fs::File;
use std::io::{self, BufRead, stdin, BufReader};
use std::io::Write;
use std::ptr::null;
use std::ptr::null_mut;
use std::vec;
use std::fmt;
use std::process;



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
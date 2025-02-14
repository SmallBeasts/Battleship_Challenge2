use std::fs::File;
use std::io::{self, BufRead, BufReader};
use crate::code::board::GameData;
use crate::code::utils::output_string;
use crate::code::utils::parse_to_usize;
use std::collections::{HashSet, HashMap};
use std::error::Error;
use crate::code::board::ShipBoundingBox;
use std::fmt;

#[derive(Debug)]
struct LoadError {
    message: String,
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for LoadError {} // Implement the Error trait

fn load_file_game_data(line: &str, myboard: &mut GameData, line_num: usize) -> Result<(), String> {
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
                    match err {
                        RowColErr::TooSmall => {
                            let fail_str = "Error: Failed to load player correctly, too small.".to_string();
                        }
                        RowColErr::TooBig => {
                            let fail_str = "Error: Failed to load player correctly, too big.".to_string();
                        }
                        RowColErr::Failed => { let fail_str = "Error: Failed to load player correctly, failed.".to_string();
                        }   
                    }
                    return Err(fail_str);
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
                    match err {
                        RowColErr::TooSmall => {
                            let fail_str = "Error: Failed to load player correctly, too small.".to_string();
                        }
                        RowColErr::TooBig => {
                            let fail_str = "Error: Failed to load player correctly, too big.".to_string();
                        }
                        RowColErr::Failed => { let fail_str = "Error: Failed to load player correctly, failed.".to_string();
                        }   
                    }
                    return Err(fail_str);
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
                    match err {
                        RowColErr::TooSmall => {
                            let fail_str = "Error: Failed to load player correctly, too small.".to_string();
                        }
                        RowColErr::TooBig => {
                            let fail_str = "Error: Failed to load player correctly, too big.".to_string();
                        }
                        RowColErr::Failed => { let fail_str = "Error: Failed to load player correctly, failed.".to_string();
                        }   
                    }
                    return Err(fail_str);
                }
            }
        }
        _ => {
            return Err("Went too far, not sure why.".to_string())
        }
    }
}

// Pass the player data in as a whole, so iterate through.
pub fn load_player_game_data<R: BufRead>(
    lines: &mut std::io::Lines<R>,
    myboard: &mut GameData,
) -> Result<(), Box<dyn std::error::Error>> {
    let (play_row, play_col) = myboard.get_row_col();
    let mut player_num = 0;

    while let Some(Ok(player_name_line)) = lines.next() { // Outer loop for each player
        let player_name = player_name_line.trim();

        if player_name.contains(',') || player_name.is_empty() {
            return Err(Box::new(LoadError {
                message: "Error: Inappropriate or blank player name.".to_string(),
            }));
        }

        let mut player = PlayBoard::default();
        player.set_playername(player_name.to_string());
        player.set_playernum(player_num);
        player_num += 1;

        let mut tmp_ships_hash: HashMap<usize, HashSet<(usize, usize)>> = HashMap::new();
        let mut current_row: Vec<usize> = Vec::new();
        let mut row_index = 0;

        for _ in 0..play_row { // Inner loop for each row of the board
            if let Some(Ok(line)) = lines.next() {
                current_row.clear();
                let row_str = line.split(',');
                for s in row_str{
                    let val = s.trim().parse().unwrap_or(0);
                    current_row.push(val);
                }

                if current_row.len() > play_col {
                    return Err(Box::new(LoadError {
                        message: format!("Error: Too many columns in row {} of player {}", row_index + 1, player_num -1).to_string(),
                    }));
                }

                for (col_index, &cell) in current_row.iter().enumerate() {
                    if cell != 0 {
                        tmp_ships_hash.entry(cell).or_insert_with(HashSet::new).insert((row_index, col_index));
                    }
                }

                row_index += 1;
            } else {
                return Err(Box::new(LoadError {
                    message: format!("Error: Not enough rows for player {}", player_num).to_string(),
                }));
            }
        }

        // Process all ships for the current player
        for (ship_id, ship_parts) in tmp_ships_hash.iter() {
            // ... (min/max calculation and BoundingBox creation - same as before)
            let mut min_row = usize::MAX;
            let mut max_row = usize::MIN;
            let mut min_col = usize::MAX;
            let mut max_col = usize::MIN;

            for &(row, col) in ship_parts.iter() {
                min_row = min_row.min(row);
                max_row = max_row.max(row);
                min_col = min_col.min(col);
                max_col = max_col.max(col);
            }

            let direction = if max_col > min_col {
                crate::Direction::Horizontal
            } else {
                crate::Direction::Vertical
            };

            let ship = ShipBoundingBox::new(
                *ship_id,
                (min_col, min_row),
                direction,
                myboard,
            );
            if let Some(s) = ship{
                player.add_ship(s);
            }
        }

        myboard.boards_add(player); // Add the player with their ships
    }

    Ok(())
}
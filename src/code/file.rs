use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::code::board::GameData;
use crate::code::utils::{output_string, parse_to_usize, handle_file_error};
use crate::code::enums::RowColErr;
use crate::code::board::PlayBoard;
use crate::code::enums::Direction;
use std::collections::{HashSet, HashMap};
use std::error::Error;
use crate::code::board::ShipBoundingBox;
use std::fmt;
use crate::code::board;

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

pub fn load_file_game_data(line: &str, myboard: &mut GameData, line_num: usize) -> Result<(), String> {
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
                    let mut fail_str = "".to_string();
                    match err {
                        RowColErr::TooSmall => {
                            fail_str = "Error: Failed to load player correctly, too small.".to_string();
                        }
                        RowColErr::TooBig => {
                            fail_str = "Error: Failed to load player correctly, too big.".to_string();
                        }
                        RowColErr::Failed => {fail_str = "Error: Failed to load player correctly, failed.".to_string();
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
                    let mut fail_str = "".to_string();
                    match err {
                        RowColErr::TooSmall => {
                            fail_str = "Error: Failed to load player correctly, too small.".to_string();
                        }
                        RowColErr::TooBig => {
                            fail_str = "Error: Failed to load player correctly, too big.".to_string();
                        }
                        RowColErr::Failed => {fail_str = "Error: Failed to load player correctly, failed.".to_string();
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
                    let mut fail_str = "".to_string();
                    match err {
                        RowColErr::TooSmall => {
                            fail_str = "Error: Failed to load player correctly, too small.".to_string();
                        }
                        RowColErr::TooBig => {
                            fail_str = "Error: Failed to load player correctly, too big.".to_string();
                        }
                        RowColErr::Failed => {fail_str = "Error: Failed to load player correctly, failed.".to_string();
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
    let (play_row, play_col) = myboard.get_col_row();
    let mut player_num = 0;

    while let Some(Ok(player_name_line)) = lines.next() {
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

        for _ in 0..play_row {
            if let Some(Ok(line)) = lines.next() {
                current_row.clear();
                let row_str = line.split(',');
                for s in row_str {
                    let val = s.trim().parse().unwrap_or(0);
                    current_row.push(val);
                }

                if current_row.len() > play_col {
                    return Err(Box::new(LoadError {
                        message: format!(
                            "Error: Too many columns in row {} of player {}",
                            row_index + 1,
                            player_num - 1
                        ),
                    }));
                }

                for (col_index, &cell) in current_row.iter().enumerate() {
                    if cell != 0 {
                        tmp_ships_hash
                            .entry(cell)
                            .or_insert_with(HashSet::new)
                            .insert((row_index, col_index));
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
        process_ships(&tmp_ships_hash, &mut player, myboard)?;

        myboard.boards_add(player);
    }

    Ok(())
}

fn process_ships(
    tmp_ships_hash: &HashMap<usize, HashSet<(usize, usize)>>,
    player: &mut PlayBoard,
    myboard: &GameData,
) -> Result<(), Box<dyn std::error::Error>> {
    for (ship_id, ship_parts) in tmp_ships_hash.iter() {
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
            Direction::Horizontal
        } else {
            Direction::Vertical
        };

        let is_contiguous = if direction == Direction::Horizontal {
            (min_col..=max_col).all(|col| ship_parts.contains(&(min_row, col)))
        } else {
            (min_row..=max_row).all(|row| ship_parts.contains(&(row, min_col)))
        };

        if !is_contiguous || (if direction == Direction::Horizontal { max_col - min_col + 1 } else { max_row - min_row + 1 }) != *ship_id {
            return Err(Box::new(LoadError {
                message: format!("Error: Ship {} is not properly sized or has gaps", *ship_id),
            }));
        }

        let ship = ShipBoundingBox::new(*ship_id, (min_col, min_row), direction, myboard, player);
        if let Some(s) = ship {
            player.add_ship(s);
        }
    }

    Ok(())
}

pub fn load_file(filename: &str, myboard: &mut GameData) -> Result<(), String> {
    if filename.is_empty() {
        return Err("Error: Filename cannot be empty.".to_string());
    }

    let file = File::open(filename).map_err(|err| format!("Error opening file: {}", err))?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    for (line_num, line) in lines.by_ref().enumerate() {
        if line_num < 3 {
            let line_content = line.map_err(|_| format!("Error reading line {}", line_num))?;
            load_file_game_data(&line_content, myboard, line_num)?;
        } else {
            break;
        }
    }

    load_player_game_data(&mut lines, myboard)
        .map_err(|err| format!("Error loading player game data: {}", err))?;

    myboard.set_loaded(true);
    Ok(())
}


pub fn write_file(myboard: &mut GameData) -> bool {
    let (my_cols, my_rows) = myboard.get_col_row();
    let file = File::create(myboard.get_filename());
    match file {
        Err(err) => {
            handle_file_error(err);
            return false;
        }
        Ok(myfile) => {
            let mut writer = BufWriter::new(myfile);
            writeln!(writer, "{}\n{}\n{}", my_cols, my_rows, myboard.get_playercount());    // Write global data first
            while let Some(mut playboard) = myboard.remove_first_board() {
                let tmpboard = board::create_my_board_from_player(myboard, &mut playboard);             // Create a temp board to write
                writeln!(writer, "{}", playboard.get_playername());
                for row in tmpboard {
                    let mut row_str = String::new();
                    for &val in row.iter() {
                        row_str.push_str(&val.to_string());
                        row_str.push(',');
                    }
                    if !row_str.is_empty() {
                        row_str.pop();
                    }
                    writeln!(writer, "{}", row_str);
                }
            }
        }
    }
    return true;
}

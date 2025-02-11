
use crate::code::board::GameData;

// Query the array to see what is at each location.  This is mainly for the challenge
pub fn query_array(mybuf: &str, myboard: &mut GameData, playerid: i16) -> Result<i16, String> {
    let mut row_str = String::new();
    let mut col_str = String::new();
    if myboard.get_loaded() == false {
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
        return Err(format!("Invalid query (missing row or column): {}", mybuf).to_string());
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
/*Deprecated since now no longer storing rows 
    let (real_row, real_col) = myboard.get_row_col();
    if row_index <=  real_row as usize && col_index <= real_col as usize{
        match myboard.boards_get_player(playerid as usize) {
            Some(value) => {
                match value.get_board_loc(row_index, col_index) {
                    Some(val) => Ok(val),
                    None => Err(format!("Error: Unable to find data at {}", mybuf).to_string())
                }
            },
            None => {
                Err(format!("Error: No data at point {}, {} in player {}, correlating to {}",
                     row_index, col_index, playerid, mybuf).to_string())
            }
        }
    }
    else {
        Err(format!("Query out of bounds: Row {}, Column {}", row_index + 1, col_index + 1).to_string())
    }
    */
    // Fix this, but I think query_array is completely done, likely going to only return hit/miss
    Ok(1)
}
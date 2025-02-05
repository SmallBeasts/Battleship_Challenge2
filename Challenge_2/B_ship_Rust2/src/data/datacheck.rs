use std::fs::File;
use std::io::{self, BufRead, stdin, BufReader};
use std::io::Write;
use std::ptr::null;
use std::ptr::null_mut;
use std::vec;
use std::fmt;
use std::process;


// Query the array to see what is at each location.  This is mainly for the challenge
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
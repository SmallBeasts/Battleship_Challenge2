use std::fs::File;
use std::io::{self, BufRead, stdin, BufReader};
use std::io::Write;
use std::ptr::null;
use std::ptr::null_mut;
use std::vec;
use std::fmt;
use std::process;

// Function that allows for consistent output--Pretty
fn output_string(buf: &str) {
    print!("{}\n:> ", buf);
}

// Handling repetitive file errors
fn handle_file_error(err: io::Error) -> bool {
    output_string(&format!("Error: Failed to open specified file: {}", err));
    false
}

// Handle repetitive parsing errors when converting string to int
fn handle_parse_error(err: &str) -> bool {
    output_string(&format!("Error: Failed to parse value: {}", err));
    false
}

fn handle_player_parse_error(err: &str) -> bool {
    output_string(&format!("Error: Failed to load player correctly: {}", err));
    false
}

// Function to return base 26 for the columns
fn base_26(buf: String) -> i16 {
    let mut col_index: i16 = 0;
    for c in buf.chars() {
        col_index = col_index * 26 + (c as u8 - b'A') as i16;
    }
    col_index
}

fn parse_to_int(mystr: &str) -> Result<i16, RowColErr> {
    match mystr.parse::<i16>() {
        Ok(n) => {
            if n <= 0 {
                return Err(RowColErr::TooSmall);
            }
            else {
                Ok(n)
            }
        },
        Err(_) => Err(RowColErr::Failed),
    }
}

// Function to translate a Column Row notation into a query notation
fn translate_query(mybuf: &str) -> Result<(i16, i16), QueryError> {
    let mut row_raw = String::new();
    let mut col_raw = String::new();

    for c in mybuf.to_uppercase().chars() {
        if c.is_ascii_uppercase() {
            col_raw.push(c);
        } else if c.is_ascii_digit() {
            row_raw.push(c);
        } else if c.is_whitespace() {
            continue;
        }
    }

    if row_raw.is_empty() || col_raw.is_empty() {
        return Err(QueryError::InvalidFormat);
    }

    let mut col_index = base_26(col_raw);

    let row_index = row_raw.parse::<i16>().map_err(|_| QueryError::InvalidRow)?;

    Ok((col_index, row_index))
}
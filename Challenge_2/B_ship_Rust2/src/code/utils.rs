use crate::code::enums::QueryError;
use crate::code::enums::RowColErr;
use std::io;
use std::fmt::Display;
use crate::code::enums;

// Function that allows for consistent output--Pretty
pub fn output_string<T: Display + ?Sized>(buf: &T) {
    let x = format!(":> {}\n:> ", buf);
    print!("{}", x);
}


// Handling repetitive file errors
pub fn handle_file_error(err: io::Error) -> bool {
    output_string(&format!("Error: Failed to open specified file: {}", err));
    false
}

// Handle repetitive parsing errors when converting string to int
pub fn handle_parse_error(err: &str) -> bool {
    output_string(&format!("Error: Failed to parse value: {}", err));
    false
}

pub fn handle_player_parse_error(err: &str) -> bool {
    output_string(&format!("Error: Failed to load player correctly: {}", err));
    false
}

// Function to return base 26 for the columns
pub fn base_26(buf: String) -> i16 {
    let mut col_index: i16 = 0;
    for c in buf.chars() {
        col_index = col_index * 26 + (c as u8 - b'A') as i16;
    }
    col_index
}

pub fn parse_to_usize(mystr: &str) -> Result<usize, RowColErr> {
    match mystr.parse::<usize>() {
        Ok(n) => {
            if n <= 0 {
                return Err(RowColErr::TooSmall);
            }
            if n > enums::MAX_SIZE {
                return Err(RowColErr::TooBig);
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

    let col_index = base_26(col_raw);

    let row_index = row_raw.parse::<i16>().map_err(|_| QueryError::InvalidRow)?;

    Ok((col_index, row_index))
}
use crate::code::enums::QueryError;
use crate::code::enums::RowColErr;
use std::io;
use std::fmt::Display;
use crate::code::enums;
use crate::code::board::GameData;
use crate::code::board;
use rand::Rng;

// Function that allows for consistent output--Pretty
pub fn output_string<T: Display + ?Sized>(buf: &T) {
    let x = format!(":> {}\n:> ", buf);
    print!("{}", x);
}

// Pick random in vectors
pub fn pick_random<T>(vec: &[T]) -> Option<&T> {
    if vec.is_empty() {
        return None;
    }
    let idx = rand::random_range(0..vec.len());
    Some(&vec[idx])
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
pub fn base_26(buf: String) -> usize {
    let mut col_index: usize = 0;
    for c in buf.chars() {
        col_index = col_index * 26 + (c as u8 - b'A') as usize;
    }
    col_index
}

// Convert a number to a base-26 letter string (A=0, B=1, ..., Z=25)
pub fn base26_to_letter(mut num: usize) -> String {
    let mut result = String::new();
    while num >= 0 {
        result.push((b'A' + (num % 26) as u8) as char);
        if num < 26 {
            break;
        }
        num = (num / 26) - 1; // Adjust for zero-based indexing
    }
    result.chars().rev().collect()
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
pub fn translate_query(mybuf: &str) -> Result<(usize, usize), QueryError> {
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

    let row_index = row_raw.parse::<usize>().map_err(|_| QueryError::InvalidRow)?;

    Ok((col_index, row_index))
}

fn create_the_header(max_col_letter: &str) -> Vec<String> {
    let mut headers = vec!["".to_string()]; // Start with a blank column

    let mut col = "A".to_string(); // Start with "A"

    while col <= max_col_letter.to_string() {
        headers.push(col.clone());
        if !next_column(&mut col) {
            break;
        }
    }

    headers
}

fn next_column(col: &mut String) -> bool {
    let mut bytes = col.as_bytes().to_vec();
    let mut i = bytes.len();

    while i > 0 {
        i -= 1;
        if bytes[i] < b'Z' {
            bytes[i] += 1; // Increment character (e.g., 'A' -> 'B')
            *col = String::from_utf8(bytes.clone()).unwrap();
            return true;
        } else {
            bytes[i] = b'A'; // Reset to 'A' and carry over
        }
    }

    // If all letters were 'Z', append an additional 'A' (e.g., "ZZ" -> "AAA")
    col.insert(0, 'A');
    true
}

// Display the board with headers and proper ANSI colors
pub fn display_board(myboard: &GameData, tmpboard: &Vec<Vec<usize>>, pad: usize) {
    let reset = "\x1b[0m"; // Reset color

    // ANSI escape sequences for colors
    let blue_bg_white_fg = "\x1b[48;5;4m\x1b[38;5;15m"; // Blue background, White text
    let darkgray_bg_yellow_fg = "\x1b[48;5;8m\x1b[38;5;11m"; // Dark Gray background, Yellow text

    let (max_col, max_row) = myboard.get_col_row();
    let max_col_letter = base26_to_letter(max_col);
    let headers = create_the_header(&max_col_letter);

    // Print column headers
    print!("{}{:width$}{}", blue_bg_white_fg, "", reset, width = pad); // Blank column
    for header in &headers[1..] {
        print!("{}{:width$}{}", blue_bg_white_fg, header, reset, width = pad);
    }
    println!();

    // Print board rows with row headers
    for (count, row) in tmpboard.iter().enumerate() {
        print!("{}{:width$}{}", blue_bg_white_fg, count + 1, reset, width = pad); // Row header

        for cell in row {
            print!("{}{:width$}{}", darkgray_bg_yellow_fg, cell, reset, width = pad);
        }
        println!();
    }
}
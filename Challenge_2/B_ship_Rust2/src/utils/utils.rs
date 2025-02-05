use std::fmt;

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
use std::collections::btree_map::Values;
use std::fs::File;
use std::io::{self, BufRead, stdin, BufReader};
use std::io::Write;
use std::ptr::null;
use std::ptr::null_mut;
use std::vec;
use std::fmt;
use std::process;


struct Board {
    rows: i16,
    cols: i16,
    loaded: bool,
    interactive: bool,
    filename: String,
    mine: Vec<Vec<i16>>,
    opp: Vec<Vec<i16>>
}

fn output_string(buf: &str) {
    print!("{}\n:> ", buf);
    io::stdout().flush().unwrap();
}

fn create_board() -> Board {
    Board {
        rows: 0,
        cols: 0,
        loaded: false,
        interactive: false,
        filename: "".to_string(),
        mine: Vec::new(),
        opp: Vec::new(),
    }
}

// Handling repetitive file errors
fn handle_file_error(err: io::Error) -> bool {
    output_string(&format!("Failed to open specified file: {}", err));
    false
}

// Handle repetitive parsing errors when converting string to int
fn handle_parse_error<T>(err: &str) -> bool {
    output_string(&format!("Failed to parse value: {}", err));
    false
}

fn load_file(filename: &str, myboard: &mut Board) ->bool {
    if filename.is_empty() {             // Empty string for filename
        return false                           // Return false
    }
    match File::open(filename) {
        Err(err) => handle_file_error(err), 
        Ok(file) => {                   // File is now open, time to read
            let reader = BufReader::new(file);
            let mut lines = reader.lines();

            // First line
            let row_line = match lines.next() {
                Some(Ok(line)) => line,
                None => {
                    output_string("File is corrupt.  Missing rows.");
                    return false;
                }
                Some(Err(err)) => {
                    output_string(&format!("File is corrupt in the rows: {}", err));
                    return false;
                }
            };

            let col_line = match lines.next() {
                Some(Ok(line)) => line,
                None => {
                    output_string("File is corrupt.  Missing columns.");
                    return false;
                }
                Some(Err(err)) => {
                    output_string(&format!("File is corrupt in the columns {}.",err));
                    return false;
                }
            };

            // Convert first line to integer
            match row_line.trim().parse::<i16>() {
                Ok(num) => {
                    if num > 0 && num <= i16::MAX {                         // Confirm Rows are positive i16
                        myboard.rows = num;
                    } else {
                        output_string("Rows are sent as a negative number.");
                        return false;
                    }
                },
                Err(err) => handle_parse_error(err)
            }

            // Convert second line to integer
            match col_line.trim().parse::<i16>() {
                Ok(num) => {
                    if num > 0 && num <= i16::MAX {                     // Confirm columns are positive i16
                        myboard.cols = num;
                    } else {
                        output_string("Columns are too large or too small.");
                    }
                },
                Err(err) => handle_parse_error(err)
            }
            myboard.mine = vec![vec![0;myboard.cols as usize]; myboard.rows as usize];       // Initialize myboard.mine


            for (i,line) in lines.enumerate().take(myboard.rows as usize) {
                if i as i16 > myboard.rows {
                    output_string(&format!("The row is greater than the total number of rows {}", i));
                    return false;
                }
                match line {
                        Ok(line_str) => {
                        let parts: Vec<&str> = line_str.split(',').collect();
                        for (j,part) in parts.iter().enumerate() {
                            if j as i16 > myboard.cols {
                                output_string(&format!("Column {} is greater than {}", j, myboard.cols));
                                return false;
                            }
                            match part.trim().parse::<i16>() {
                                Ok(val) => {
                                    if i < myboard.mine.len() && j < myboard.mine[i].len() && (val <= i16::MAX && val >= i16::MIN) {
                                        myboard.mine[i][j] = val;
                                    }
                                    else {
                                        output_string(&format!("OOB at Column {}, on row {}", j, i));
                                        return false;
                                    }
                                }
                                Err(e) => {
                                    output_string(&format!("Failed to parse column index: {}", e));
                                    return false;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        output_string(&format!("Error parsing line {}: {}", i+1, e));
                        return false;
                    }
                }
            }
            myboard.loaded = true;
            return true;
        }
    }

}

fn query_array(mybuf: &str, myboard: &mut Board) -> Result<i16, String> {
    let mut row_str = String::new();
    let mut col_str = String::new();
    if myboard.loaded == false {
        return Err("You have not loaded a file yet!".to_string());
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

    if row_index < myboard.mine.len() && col_index < myboard.mine[0].len() {
        Ok(myboard.mine[row_index][col_index])
    }
    else {
        Err(format!("Query out of bounds: Row {}, Column {}", row_index + 1, col_index + 1))
    }
}

fn parse_command(mybuf: &str, myboard: &mut Board) -> bool {
    let upper = mybuf.to_uppercase();
    let mut tokens = upper.split_whitespace();
    let command = tokens.next();
    match command {                                         // Tokenize and split the command
        Some("LOAD") => {
            if myboard.loaded {
                output_string("A previous board was loaded, now loading new file.");
                *myboard = create_board();
            }
            if let Some(filename) = tokens.next() {
                output_string(&format!("Loading the file <{}>", filename));
                if load_file(filename, myboard) {
                    output_string("File loaded successfully");
                } else {
                    output_string("File failed to load");
                }
            } else {
                output_string("Usage: --load <filename>");
            }
            false
        }
        Some("GUESS") => {
            if myboard.loaded == false {        // Not initialized with a load
                output_string("You have not loaded a file yet.");
                return false;
            }
            let mut results = Vec::new();
            let mut oob_messages = Vec::new();

            for tok in tokens {
                let guesses: Vec<&str> = tok.split(',').collect();
                for guess in guesses {
                    match query_array(guess, myboard) { // Call query_array directly
                        Ok(value) => results.push(value.to_string()),
                        Err(msg) => {
                            results.push("OOB".to_string());
                            oob_messages.push(msg);
                        }
                    }
                }
            }

            if !oob_messages.is_empty() {
                for msg in oob_messages {
                    output_string(&msg);
                }
            }
            output_string(&format!("Results are {}", results.join(",")));
            return false;
        },
        Some("HELP") => {
            output_string("Available commands: --load <filename>\n--guess <list in A1 or AA10 format>\n--help this output\n--exit or --quit to quit.");
            false
        }
        Some("EXIT") | Some("QUIT") => {
            output_string("Thank you for enjoying Battleship Test in Rust!");
            true
        }
        None => {
            output_string("No command was recognized type --help for a list of commands");
            return false
        }
        _ => {                  // Handle only commands that are not known commands 
            match query_array(upper.as_str(), myboard) {
                Ok(value) => {
                    output_string(&format!("{}", value));
                    return false;
                },
                Err(msg) => {
                    output_string(&msg);
                    return false;
                }
            }
        }
    }
}
    
        
fn eval_input(mybuf: String, myboard: &mut Board) -> bool {
    if mybuf.contains("--") {
        let tokens = mybuf.split("--").filter(|&x| !x.is_empty());     // Split on token command
        for tok in tokens {
            if parse_command(tok, myboard) {
                return true;
            }
        }
    } else {
        if parse_command(mybuf.as_str(), myboard) {
            return true;
        }
    }
    false
}

// Specific enum to give individual instances as errors.
enum QueryError {
    InvalidFormat,
    InvalidRow,
    InvalidColumn,
    OutOfBounds,
}

// Function to return base 26 for the columns
fn base_26(buf: String) -> i16 {
    let mut col_index: i16 = 0;
    for c in buf.chars() {
        col_index = col_index * 26 + (c as u8 - b'A') as i16;
    }
    col_index
}

// Function to translate a Column Row notation into a query notation
fn translate_query(mybuf: &str) -> Result<(i16, i16), String> {
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

// This function only is for the command line
fn command_line_input(myboard: &mut Board) {
    let args: Vec<String> = std::env::args().collect();
    let mut args_iter = args.iter().skip(1); // Skip program name

    while let Some(arg) = args_iter.next() {
        match arg.to_uppercase().as_str() {
            "--LOAD" => {
                if myboard.loaded {
                    *myboard = create_board();
                }
                if let Some(filename) = args_iter.next() {
                    if load_file(filename, myboard) {
                        output_string("File loaded successfully.");
                    } else {
                        output_string("File was not found, please enter the full path.");
                    }
                } else {
                    output_string("Usage: --load <filename>");
                }
            }
            "--HELP" => {
                output_string(
                    "Available commands: \
                     --load <filename>\n--guess <list in A1 or AA10 format>\n--help (this output)\n--exit or --quit to quit.",
                );
            }
            "--EXIT" | "--QUIT" => {
                output_string("Thank you for enjoying Battleship Test Rust version 1.");
                break;
            }
            "--GUESS" => {
                let mut guesses = String::new();

                // Process guesses until we encounter a new command or run out of arguments
                while let Some(next_guess) = args_iter.next() {
                    if next_guess.starts_with("--") {
                        break; // Stop processing guesses
                    }
                    match translate_query(next_guess) {
                        Ok((col, row)) => {
                            if col >= myboard.cols as i16 || row >= myboard.rows as i16 {
                                guesses.push_str("OOB,"); // Out of bounds
                            } else {
                                guesses.push_str(&format!("{},", myboard.mine[row as usize][col as usize]));
                            }
                        }
                        Err(err) => {
                            match err {
                                QueryError::InvalidFormat => output_string("Invalid guess format"),
                                QueryError::InvalidRow => output_string("Invalid row format"),
                                QueryError::InvalidColumn => output_string("Invalid column format"),
                                QueryError::OutOfBounds => output_string("Guess is out of bounds"),
                            }
                        }
                    }
                }

                if !guesses.is_empty() {
                    guesses.pop(); // Remove trailing comma
                }
                output_string(&guesses);
            }
            _ => {
                output_string(&format!("Unknown command: {}", arg));
            }
        }
    }
}


fn main(){
    output_string("Welcome to the Battleship Test Program\nYou can type --help to get a list of commands");
    let mut myboard = create_board();
    if std::env::args().len() <= 1 {
        output_string("No command line arguments entered.");
        myboard.interactive = true;
    } else {
        command_line_input(&mut myboard);
    }
    if myboard.interactive == true {                                // Only enter loop if interactive set
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            if eval_input(buffer, &mut myboard) {
                break;
            }
        }
    }
}

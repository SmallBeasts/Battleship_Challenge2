use std::collections::btree_map::Values;
use std::fs::File;
use std::io::{self, BufRead, stdin, BufReader};
use std::io::Write;
use std::ptr::null;
use std::ptr::null_mut;
use std::vec;
use std::fmt;


struct Board {
    rows: i16,
    cols: i16,
    loaded: bool,
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
        filename: "".to_string(),
        mine: Vec::new(),
        opp: Vec::new(),
    }
}

fn load_file(filename: &str, myboard: &mut Board) ->bool {
    if filename.is_empty() {             // Empty string for filename
        return false                           // Return false
    }
    match File::open(filename) {
        Err(err) => {
            output_string(&format!("Failed to open specified file: {}", err));
            return false
        },
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
                Ok(num) => myboard.rows = num,
                Err(err) => {
                    output_string(&format!("Failed to convert row to an integer {}.", err));
                    return false;
                }
            }

            // Convert second line to integer
            match col_line.trim().parse::<i16>() {
                Ok(num) => myboard.cols = num,
                Err(err) => {
                    output_string(&format!("Failed to convert columns to an integer {}.", err));
                    return false;
                }
            }
            myboard.mine = vec![vec![0;myboard.cols as usize]; myboard.rows as usize];       // Initialize myboard.mine


            for (i,line) in lines.enumerate().take(myboard.rows as usize) {
                match line {
                    Ok(line_str) => {
                        let parts: Vec<&str> = line_str.split(',').collect();
                        for (j,part) in parts.iter().enumerate() {
                            match part.trim().parse::<i16>() {
                                Ok(val) => {
                                    if i < myboard.mine.len() && j < myboard.mine[0].len() && (val <= i16::MAX && val >= i16::MIN) {
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

            
            
            
            // Debug stuff
            myboard.loaded = true;
            println!("Myboard rows is: {}", myboard.rows);
            println!("Myboard cols is: {}", myboard.cols);
            println!("Myboard mine is: {:?}", myboard.mine);
            return true;
        }
    }

}

fn parse_command(mybuf: &str, myboard: &mut Board) {
    let upper = mybuf.to_uppercase();
    let mut tokens = upper.split_whitespace();
    let command = tokens.next();

    match command {
        Some("LOAD") => {
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
        }
        Some("GUESS") => {
            for tok in tokens {
                let guesses: Vec<&str> = tok.split(',').collect();
                for guess in guesses {
                    output_string(&format!("Checking array at {}:", guess));      // Query only the mine column now
                }
            }
        }
        Some("HELP") => {
            output_string("Available commands: --load <filename>\n--guess <list in A1 or AA10 format>\n--help this output\n--exit or --quit to quit.");
        }
        Some(cmd) => {
            output_string(&format!("Command not recognized: {}?", cmd));
        }
        None => {
            output_string("Available commands: --load <filename>\n--guess <list in A1 or AA10 format>\n--help this output\n--exit or --quit to quit.");
        },
    }
}
    
        
fn eval_input(mybuf: String, myboard: &mut Board) {
    if mybuf.contains("--") {
        let tokens = mybuf.split("--").filter(|&x| !x.is_empty());     // Split on token command
        for tok in tokens {
            parse_command(tok, myboard);
        }
    } else {
        println!("my buf is: {:?}", mybuf);
    }
    
}

fn main(){
    output_string("Welcome to the Battleship Test Program\nYou can type --help to get a list of commands");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut myboard = create_board();
    eval_input(buffer, &mut myboard);

}

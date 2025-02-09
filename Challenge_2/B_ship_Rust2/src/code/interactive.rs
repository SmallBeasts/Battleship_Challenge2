use crate::code::board::GameData;
use crate::code::utils::output_string;


// This is still present for interactive mode but out of date on most commands.
fn parse_command(mybuf: &str, myboard: &mut GameData) -> bool {
    let upper = mybuf.to_uppercase();
    let mut tokens = upper.split_whitespace();
    let command = tokens.next();
    match command {                                         // Tokenize and split the command
        Some("LOAD") => {
            if myboard.get_loaded() {
                output_string("A previous board was loaded, now loading new file.");
                *myboard = board::create_game();
            }
            if let Some(filename) = tokens.next() {
                output_string(&format!("Loading the file <{}>", filename));
                if file::load_file(filename, myboard) {
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
            if myboard.get_loaded() == false {        // Not initialized with a load
                output_string("You have not loaded a file yet.");
                return false;
            }
            let mut results = Vec::new();
            let mut oob_messages = Vec::new();

            for tok in tokens {
                let mut tmp_id: i16 = -1;
                let guesses: Vec<&str> = tok.split(',').collect();
                for (guess_num, guess) in guesses.iter().enumerate() {
                    if guess_num == 0 {
                        match guess.parse::<i16>() {
                            Ok(n) => {
                                if n > myboard.get_playercount() || n < 1 {
                                    output_string(&format!("Error: Invalid playerid, {}", n));
                                    return false;
                                }
                                else {
                                    tmp_id = n;
                                }
                            }
                            Err(_) => {
                                output_string(&format!("Error: Invalid playerid, {}", guess));
                                return false;
                            }
                        }
                    }
                    match query_array(guess, myboard, tmp_id) { // Call query_array directly
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
        _ => {                  // Handle only commands that are not known commands 0 is default player
            match code::datacheck::query_array(upper.as_str(), myboard, 0) {
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


// This command is called in interactive mode from main.
pub fn eval_input(mybuf: String, myboard: &mut GameData) -> bool {
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
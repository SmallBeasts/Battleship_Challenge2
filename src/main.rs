use std::fs;
use std::io::{self, BufRead};
use std::io::Write;
use std::ptr::null;
use std::ptr::null_mut;
use std::vec;


struct Board {
    rows: i16,
    cols: i16,
    loaded: bool,
    filename: String,
    mine: Vec<i16>,
    opp: Vec<i16>
}

fn output_string(buf: &str) {
    print!("{}\n:> ", buf);
    io::stdout().flush().unwrap();
}

fn create_board(filename: String) -> Board {
    Board {
        rows: 0,
        cols: 0,
        loaded: false,
        filename,
        mine: Vec::new(),
        opp: Vec::new(),
    }
}

fn parse_command(mybuf: &str, &mut myboard: Board) {
    let tokens = mybuf.split_whitespace();
    let mut state = 0;
    for tok in tokens {
        if tok == "load" {
            state = 1;
            continue;
        }
        if tok == "guess" {
            state = 2;
            continue;
        }
        if state == 1 {     // Loading a file so the next token is the filename
            println!("Loading the file <{}>", tok);
            continue;
        }
        if state == 2 {     // Guess so load up query
            
        }
    }
}
fn eval_input(mybuf: String) {
    if mybuf.contains("--") {
        let tokens = mybuf.split("--").filter(|&x| !x.is_empty());     // Split on token command
        for tok in tokens {
            println!("What up?! {:?}", tok);
        }
    } else {
        println!("my buf is: {:?}", mybuf);
    }
    
}

fn main(){
    output_string("Welcome to the Battleship Test Program\nYou can type --help to get a list of commands");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let myboard = create_board(filename);
    eval_input(buffer);

}

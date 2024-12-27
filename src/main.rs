use std::fs;
use std::io;
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

fn parse_input(mybuf: String) {
    let tokens = mybuf.split("--");     // Split on token command
    for tok in tokens {
        println!("{:?}");
    }

}

fn main() {
    output_string("Welcome to the Battleship Test Program\nYou can type --help to get a list of commands");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    parse_input(buffer);

}

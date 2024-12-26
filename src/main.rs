use std::fs;
use std::io;
use std::io::Write;
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

fn main() {
    output_string("Welcome to the Battleship Test Program\nYou can type --help to get a list of commands");


}

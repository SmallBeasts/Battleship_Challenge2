use std::fs::File;
use std::io::{self, BufRead, stdin, BufReader};
use std::io::Write;
use std::ptr::null;
use std::ptr::null_mut;
use std::vec;
use std::fmt;
use std::process;

mod game;   // Import the board and functions related to structures
mod utils;
mod file;
mod input;
mod data;


fn main(){
    output_string("Welcome to the Battleship Test Program v2\nYou can type --help to get a list of commands");
    let mut myboard = create_game();
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

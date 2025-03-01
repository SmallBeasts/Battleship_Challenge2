use std::fs::File;
use std::io::{self, BufRead, stdin, BufReader};
use std::vec;
use crate::code::utils;
use crate::code::commandline;
use crate::code::interactive;
use crate::code::board;
mod code;


fn main(){
    utils::output_string("Welcome to the Battleship Test Program\nYou can type --help to get a list of commands");
    let mut myboard = board::GameData::default();
    if std::env::args().len() <= 1 {
        utils::output_string("No command line arguments entered.");
        myboard.set_interactive(true);
    } else {
        commandline::command_line_input(&mut myboard);
    }
    if myboard.get_interactive() == true {                                // Only enter loop if interactive set
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            if interactive::eval_input(buffer, &mut myboard) {
                break;
            }
        }
    }
}

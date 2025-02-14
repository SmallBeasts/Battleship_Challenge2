use std::io;

mod code;       // Load in the actual module code


fn main(){
    code::utils::output_string("Welcome to the Battleship Test Program v2\nYou can type --help to get a list of commands");
    let mut myboard = code::board::GameData::default();
    if std::env::args().len() <= 1 {
        code::utils::output_string("No command line arguments entered.");
        myboard.set_interactive(true);
    } else {
        code::commandline::command_line_input(&mut myboard);
    }
    if myboard.get_interactive() {                                // Only enter loop if interactive set
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            if code::interactive::eval_input(buffer, &mut myboard) {
                break;
            }
        }
    }
}

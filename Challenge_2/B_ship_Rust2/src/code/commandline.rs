use crate::code::board::GameData;
use crate::code::enums::StateCreate;
use crate::code::handler;
use crate::code::utils::output_string;

// Refactored command line function
pub fn command_line_input(myboard: &mut GameData) -> bool{
    let args: Vec<String> = std::env::args().collect();
    let mut args_iter = args.iter().skip(1);        // Skip program name
    let mut mystate = Vec::<StateCreate>::new();              // Keep track of the state of Create commands

    while let Some(arg) = args_iter.next() {
        match arg.to_uppercase().as_str() {
            "--LOAD" => {
                handler::handle_load(myboard, &mut args_iter, false);           
            },
            "--VERIFY" => {
                handler::handle_load(myboard, &mut args_iter, true);
            },
            "--HELP" => {
                handler::handle_help();
            }
            "--EXIT" | "--QUIT" => {
                output_string("Thank you for playing!");
                break;
            },
            "--GUESS" => {
                handler::handle_guess(myboard, &mut args_iter);
            },
            "--CREATE" => {
                if handler::handle_create(myboard, &mut args_iter, &mut mystate) {      // Successful file create
                    continue;
                } else {
                    break;
                }
            },
            "--ROW" => {
                if let Err(err) = handler::handle_row_col(myboard, &mut args_iter, true) {
                    handler::handle_row_col_error(err, true);
                }
            },
            "--COL" => {
                if let Err(err) = handler::handle_row_col(myboard, &mut args_iter, false) {
                    handler::handle_row_col_error(err, false);
                }
            },
            "--SHIPS" => {
                handler::handle_ships_size(myboard, &mut args_iter, &mut mystate);
            },
            "--PLAYER" => {
                handler::handle_player(myboard, &mut args_iter, &mut mystate);
            },
            "--RANDOM" => {
                handler::handle_random(myboard, &mut mystate);
            },
            "--DISPLAY" => {
                handler::handle_display(myboard);
            },
            "--PLACE" => {
                handler::handle_place_ship(myboard, &mut args_iter, &mut mystate);
            },
            &_ => {
                output_string("Error: Unrecognized command.");
                break;
            }
        }
    };
    if mystate.contains(&StateCreate::StateCreate) {                 // We need to write the file at the end
        handler::handle_write_file(myboard);
    }
    true
}

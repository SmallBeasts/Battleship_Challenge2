use std::fs::File;
use std::io::{self, BufRead, stdin, BufReader};
use std::io::Write;
use std::ptr::null;
use std::ptr::null_mut;
use std::vec;
use std::fmt;
use std::process;

use create::utils;

fn handle_row_col_error(err: RowColErr, is_row: bool) {
    let err_msg = match err {
        RowColErr::Failed => {
            if is_row {
                "Error: Row is not a valid value"
            } else {
                "Error: Column is not a valid value"
            }
        }
        RowColErr::TooSmall => {
            if is_row {
                "Error: Row value must be greater than or equal to 1"
            } else {
                "Error: Column value must be greater than or equal to 1"
            }
        }
    };
    output_string(err_msg);
}

// Handle conversion and storage of row and column data in Create specifically but later probably in load.
fn handle_row_col(
    myboard: &mut GameData, 
    args_iter: &mut std::iter::Skip<std::slice::Iter<String>>, 
    row_col: bool
) -> Result<(), RowColErr> {
    if let Some(next_value) = args_iter.next() {
        match create_row_col(next_value) {
            Ok(value) => {
                if row_col {
                    myboard.rows = value;
                } else {
                    myboard.cols = value;
                }
                return Ok(());
            }
            Err(err) => return Err(err),
        }
    } else {
        return Err(RowColErr::Failed);
    }
}


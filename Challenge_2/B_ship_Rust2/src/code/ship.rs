use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// ... (Your BoundingBox and other definitions) ...

fn load_board_from_file(filename: &str) -> Result<Vec<BoundingBox>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut ships: Vec<BoundingBox> = Vec::new();
    let mut board_data: Vec<Vec<usize>> = Vec::new();
    let mut processed: HashSet<(usize, usize)> = HashSet::new();

    for (row_index, line) in reader.lines().enumerate() {
        let line = line?;
        let row: Vec<usize> = line
            .split(',')
            .map(|s| s.trim().parse().unwrap_or(0))
            .collect();
        board_data.push(row.clone());

        for (col_index, &cell) in row.iter().enumerate() {
            if cell != 0 && !processed.contains(&(row_index, col_index)) {
                let ship_id = cell;
                let mut start_row = row_index;
                let mut start_col = col_index;
                let mut end_row = row_index;
                let mut end_col = col_index;

                // Check horizontal first
                if col_index + 1 < row.len() && board_data[row_index][col_index + 1] == ship_id {
                    let mut j = col_index;
                    while j < row.len() && board_data[row_index][j] == ship_id {
                        end_col = j;
                        processed.insert((row_index, j));
                        j += 1;
                    }
                    let ship = BoundingBox::new(
                        ship_id,
                        (start_col, start_row),
                        crate::Direction::Horizontal,
                        /* Your GameData */
                    );
                    if let Some(s) = ship{
                        ships.push(s);
                    }
                } else if row_index + 1 < board_data.len() && board_data[row_index + 1][col_index] == ship_id {
                    // If not horizontal, check vertical
                    let mut i = row_index;
                    while i < board_data.len() && board_data[i][col_index] == ship_id {
                        end_row = i;
                        processed.insert((i, col_index));
                        i += 1;
                    }
                    let ship = BoundingBox::new(
                        ship_id,
                        (start_col, start_row),
                        crate::Direction::Vertical,
                        /* Your GameData */
                    );
                    if let Some(s) = ship{
                        ships.push(s);
                    }
                } else {
                    // Single-cell ship (both horizontal and vertical checks failed)
                    let ship = BoundingBox::new(
                        ship_id,
                        (start_col, start_row),
                        crate::Direction::Horizontal, // Or Vertical, doesn't matter for single cell
                        /* Your GameData */
                    );
                    if let Some(s) = ship{
                        ships.push(s);
                    }
                    processed.insert((row_index, col_index)); // Mark even single cell as processed
                }
            }
        }
    }

    Ok(ships)
}

// ... (main function remains the same)
use std::collections::{HashMap, HashSet};


use crate::code::board;
use crate::code::enums::{Guess, Intelligence, ShipType, Direction, Mode};
use crate::code::utils::base_26;

use super::board::GameData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShipType {
    pub size: usize,
}

#[derive(Clone, Debug)]
pub struct Placement {
    pub ship: ShipType,
    pub cells: Vec<(usize, usize)>,       // Expanded list of all coordinates
    pub direction: Direction,
}

pub struct AIdata {
    pub rows: usize,
    pub cols: usize,
    pub intelligence: Intelligence,             // Level of AI

    pub mode: Mode,

    pub board_status: Vec<Vec<Guess>>,           // Board with Unknown/miss/hit
    pub heatmap: Vec<Vec<usize>>,           // 
    
    pub valid_places: HashMap<ShipType, Vec<Placement>>,        // All possible placements
    pub alive_ships: HashSet<ShipType>,

    pub unresolved_hits: Vec<(usize, usize)>,           // Hits not attached to a sunken ship
}

impl AIdata {
    pub fn new(rows:usize, cols: usize, ships: Vec<ShipType>, 
        intelligence: Intelligence) -> Self {

        let board_status = vec![vec![Guess::Unknown; cols]; rows];
        let heatmap = vec![vec![0;cols];rows];
        let mut valid_places = HashMap::new();
        let alive_ships = ships.into_iter().copied().collect();

        for &ship in ships {
            let placements = AIdata::generate_placements(rows, cols, ship);
            valid_places.insert(ship, placements);
        }
        
        Self {
           rows,
           cols,
           intelligence,
           mode: Mode::Hunt,
           board_status,
           heatmap,
           valid_places,
           alive_ships,
           unresolved_hits: Vec::new(),
        }
    }

    fn generate_placements(rows:usize, cols:usize, ship: ShipType) -> Vec<Placement> {
        let mut placements = Vec::new();

        // Horizontal first
        for r in 0..rows {
            for c in 0..=cols - ship.size {
                let cells: Vec<(usize, usize)> = (0..ship.size).map(|i| (r, c+i)).collect();
                placements.push(Placement {
                    ship, cells, direction: Direction::Horizontal,});
            }
        }
        
        // Vertical next
        for c in 0..cols {
            for r in 0..=rows - ship.size {
                let cells: Vec<(usize, usize)> = (0..ship.size).map(|i| (r+i, c)).collect();
                placements.push(Placement {
                    ship, cells, direction: Direction::Vertical,});
            }
        }
        placements
    
    }

    pub fn reset_heatmap(&mut self) {
        for row in &mut self.heatmap {
            for cell in row.iter_mut() {
                *cell = 0;
            }
        }
    }

}
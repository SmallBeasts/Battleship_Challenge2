use std::collections::HashMap;

use crate::code::board;
use crate::code::enums::HitMiss;
use crate::code::enums::Direction;
use crate::code::utils::base_26;

use super::board::GameData;

pub struct AiBoard {
    myplayer: board::PlayBoard,
    myheatmap: HashMap<String, HitMiss>,
    intelligence: usize,
}

impl AiBoard {
    pub fn new(myplayer: board::PlayBoard, intelligence: usize) -> Self {
        Self {
            myplayer: myplayer,
            myheatmap: HashMap::new(),
            intelligence: usize,
        }
    }

    pub fn create_heat_map(&self, myboard: &mut board::GameData) {
        // Still need to do this.
    }
}
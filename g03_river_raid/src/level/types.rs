use crate::prelude::*;

pub enum RiverLineType {
    Standard, // Bank on either side
}

#[derive(Copy, Clone)]
pub enum TileType {
    Water,
    BankLeft,
    BankRight,
}

impl RiverLineType {
    pub fn random_next(&self) -> Self {
        match self {
            RiverLineType::Standard => RiverLineType::Standard,
        }
    }

    pub fn get_tiles(&self) -> Vec<TileType> {
        let bl = TileType::BankLeft;
        let w = TileType::Water;
        let br = TileType::BankRight;

        match self {
            RiverLineType::Standard => vec![bl, w, w, w, w, w, br],
        }
    }
}
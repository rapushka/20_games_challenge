use crate::prelude::*;
use crate::random::Random;

#[derive(Copy, Clone)]
pub enum RiverLineType {
    Standard, // Bank on either side
    IslandsStart,
    IslandsMid,
    IslandsEnd,
}

#[derive(Copy, Clone)]
pub enum TileType {
    Water,
    BankLeftBottom,
    BankRightBottom,
    BankOutLeftMiddle,
    BankOutRightMiddle,
    BankInLeftMiddle,
    BankInRightMiddle,
    BankLeftTop,
    BankRightTop,
}

impl RiverLineType {
    pub fn random_next(&self, rand: &mut Random) -> Self {
        let standard = RiverLineType::Standard;
        let island_start = RiverLineType::IslandsStart;
        let island_mid = RiverLineType::IslandsMid;
        let island_end = RiverLineType::IslandsEnd;

        let variants = match self {
            RiverLineType::Standard => vec![standard, island_start],
            RiverLineType::IslandsStart => vec![island_mid, island_end],
            RiverLineType::IslandsMid => vec![island_mid, island_end],
            RiverLineType::IslandsEnd => vec![standard],
        };

        rand.pick(variants)
    }

    pub fn get_tiles(&self) -> Vec<TileType> {
        let wat = TileType::Water;
        let bolm = TileType::BankOutLeftMiddle;
        let borm = TileType::BankOutRightMiddle;
        let bilm = TileType::BankInLeftMiddle;
        let birm = TileType::BankInRightMiddle;
        let blb = TileType::BankLeftBottom;
        let brb = TileType::BankRightBottom;
        let blt = TileType::BankLeftTop;
        let brt = TileType::BankRightTop;

        match self {
            RiverLineType::Standard => vec![bolm, wat, wat, wat, wat, wat, wat, borm],
            RiverLineType::IslandsStart => vec![bolm, wat, wat, blb, brb, wat, wat, borm],
            RiverLineType::IslandsMid => vec![bolm, wat, wat, bilm, birm, wat, wat, borm],
            RiverLineType::IslandsEnd => vec![bolm, wat, wat, blt, brt, wat, wat, borm],
        }
    }
}
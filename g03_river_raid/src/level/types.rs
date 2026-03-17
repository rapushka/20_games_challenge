use crate::prelude::*;
use crate::random::Random;

#[derive(Copy, Clone)]
pub enum RiverLineType {
    Standard, // Bank on either side
    IslandsStart,
    IslandsMid,
    IslandsEnd,
    NarrowStart,
    NarrowMid,
    NarrowEnd,
}

#[derive(Copy, Clone)]
pub enum TileType {
    Water,

    // right
    BankOutLeftMiddle,
    BankOutLeftTop,
    BankOutCenterTop,
    BankOutRightTop,
    BankOutRightMiddle,
    BankOutLeftBottom,
    BankOutRightBottom,

    // left
    BankInLeftBottom,
    BankInRightBottom,
    BankInLeftMiddle,
    BankInRightMiddle,
    BankInLeftTop,
    BankInRightTop,
    BankInCenterMiddle,
}

impl RiverLineType {
    pub fn random_next(&self, rand: &mut Random) -> Self {
        let std = RiverLineType::Standard;
        let is = RiverLineType::IslandsStart;
        let im = RiverLineType::IslandsMid;
        let ie = RiverLineType::IslandsEnd;
        let ns = RiverLineType::NarrowStart;
        let nm = RiverLineType::NarrowMid;
        let ne = RiverLineType::NarrowEnd;

        let variants = match self {
            // standard is repeated, so it will have more chances
            RiverLineType::Standard => vec![std, std, std, is, ns],
            RiverLineType::IslandsStart => vec![im, ie],
            RiverLineType::IslandsMid => vec![im, ie],
            RiverLineType::IslandsEnd => vec![std],
            RiverLineType::NarrowStart => vec![nm, ne],
            RiverLineType::NarrowMid => vec![nm, ne],
            RiverLineType::NarrowEnd => vec![std],
        };

        rand.pick(variants)
    }

    pub fn get_tiles(&self) -> Vec<TileType> {
        let wat = TileType::Water;
        let bolm = TileType::BankOutLeftMiddle;
        let borm = TileType::BankOutRightMiddle;
        let bilm = TileType::BankInLeftMiddle;
        let birm = TileType::BankInRightMiddle;
        let bilb = TileType::BankInLeftBottom;
        let birb = TileType::BankInRightBottom;
        let bilt = TileType::BankInLeftTop;
        let birt = TileType::BankInRightTop;
        let bolt = TileType::BankOutLeftTop;
        let boct = TileType::BankOutCenterTop;
        let bort = TileType::BankOutRightTop;
        let bolb = TileType::BankOutLeftBottom;
        let borb = TileType::BankOutRightBottom;
        let bicm = TileType::BankInCenterMiddle;

        match self {
            RiverLineType::Standard => vec![bolm, wat, wat, wat, wat, wat, wat, borm],
            RiverLineType::IslandsStart => vec![bolm, wat, wat, bilb, birb, wat, wat, borm],
            RiverLineType::IslandsMid => vec![bolm, wat, wat, bilm, birm, wat, wat, borm],
            RiverLineType::IslandsEnd => vec![bolm, wat, wat, bilt, birt, wat, wat, borm],
            RiverLineType::NarrowStart => vec![bolt, birb, wat, wat, wat, wat, bilb, bort],
            RiverLineType::NarrowMid => vec![bicm, bolm, wat, wat, wat, wat, borm, bicm],
            RiverLineType::NarrowEnd => vec![bolb, birt, wat, wat, wat, wat, bilt, borb],
        }
    }
}
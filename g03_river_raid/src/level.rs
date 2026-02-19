use crate::level::types::{RiverLineType, TileType};
use crate::position::{WorldPosition, ZOrder};
use crate::prelude::*;

const TILE_SIZE: f32 = 100.0;

mod types;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Initialize), (
                spawn_level,
            ))
        ;
    }
}

fn spawn_level(
    mut commands: Commands
) {
    let mut line = RiverLineType::Standard;

    for line_index in 0..10 {
        line = line.random_next();

        let tiles = line.get_tiles();

        for (i, tile_type) in tiles.iter().enumerate() {
            let line_index = line_index as f32;
            let i = i as f32;
            let offset_x = -300f32;

            commands.spawn((
                Name::new("Tile"),
                create_bundle(*tile_type),
                WorldPosition::new(i * TILE_SIZE + offset_x, line_index * TILE_SIZE),
                ZOrder::Background,
            ));
        }
    }
}

fn create_bundle(tile_type: TileType) -> impl Bundle {
    let tile_size = Vec2::splat(TILE_SIZE);

    match tile_type {
        TileType::Water => (Sprite::from_color(utils::from_hex("#54a9ff"), tile_size)),
        TileType::BankLeft => (Sprite::from_color(utils::from_hex("#45a87d"), tile_size)),
        TileType::BankRight => (Sprite::from_color(utils::from_hex("#45a87d"), tile_size)),
    }
}
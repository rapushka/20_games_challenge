use crate::level::types::{RiverLineType, TileType};
use crate::position::{WorldPosition, ZOrder};
use crate::prelude::*;

pub use tiles::*;
use crate::random::Random;

const TILE_SIZE: f32 = 128.0;

mod types;
mod tiles;

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
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    environment: Res<EnvironmentTiles>,
    mut random: ResMut<Random>,
) {
    let mut line = RiverLineType::Standard;
    let image = asset_server.load(asset_path::ENVIRONMENT_TILESET);

    for line_index in 0..100 {
        line = line.random_next(&mut random);

        let tiles = line.get_tiles();

        for (i, tile_type) in tiles.iter().enumerate() {
            let line_index = line_index as f32;
            let i = i as f32;
            let offset_x = -450.0;

            commands.spawn((
                Name::new("Tile"),
                create_bundle(&environment, &image, *tile_type),
                WorldPosition::new(i * TILE_SIZE + offset_x, line_index * TILE_SIZE),
                ZOrder::Background,
            ));
        }
    }
}

fn create_bundle(tiles: &Res<EnvironmentTiles>, image: &Handle<Image>, tile_type: TileType) -> impl Bundle {
    let create = |atlas: TextureAtlas| {
        Sprite::from_atlas_image(
            image.clone(),
            atlas,
        )
    };

    match tile_type {
        TileType::Water => create(tiles.outer_center_center()),
        TileType::BankOutLeftMiddle => create(tiles.outer_center_left()),
        TileType::BankOutRightMiddle => create(tiles.outer_center_right()),
        TileType::BankInLeftMiddle => create(tiles.inner_center_left()),
        TileType::BankInRightMiddle => create(tiles.inner_center_right()),
        TileType::BankLeftBottom => create(tiles.inner_bottom_left()),
        TileType::BankRightBottom => create(tiles.inner_bottom_right()),
        TileType::BankLeftTop => create(tiles.inner_top_left()),
        TileType::BankRightTop => create(tiles.inner_top_right()),
    }
}
use crate::level::types::{RiverLineType, TileType};
use crate::position::{WorldPosition, ZOrder};
use crate::prelude::*;

pub use tiles::*;
use crate::collision_detection::Collider;
use crate::random::Random;

const TILE_SIZE: f32 = 128.0;

#[derive(Component)]
pub struct RiverBank;

mod types;
mod tiles;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Initialize), (
                spawn_level,
            ))

            .add_systems(OnEnter(AppState::Restart), (
                despawn_level,
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

            let x = i * TILE_SIZE + offset_x;
            let y = line_index * TILE_SIZE;

            let tile = commands.spawn((
                Name::new("Tile"),
                RiverBank,
                create_sprite(&environment, &image, *tile_type),
                WorldPosition::new(x, y),
                ZOrder::Background,
            ));

            add_collider(tile, *tile_type);
        }
    }
}

fn create_sprite(tiles: &Res<EnvironmentTiles>, image: &Handle<Image>, tile_type: TileType) -> impl Bundle {
    let texture_atlas = match tile_type {
        TileType::Water => tiles.outer_center_center(),
        TileType::BankOutLeftMiddle => tiles.outer_center_left(),
        TileType::BankOutRightMiddle => tiles.outer_center_right(),
        TileType::BankInLeftMiddle => tiles.inner_center_left(),
        TileType::BankInRightMiddle => tiles.inner_center_right(),
        TileType::BankInLeftBottom => tiles.inner_bottom_left(),
        TileType::BankInRightBottom => tiles.inner_bottom_right(),
        TileType::BankInLeftTop => tiles.inner_top_left(),
        TileType::BankInRightTop => tiles.inner_top_right(),
        TileType::BankOutLeftTop => tiles.outer_top_left(),
        TileType::BankOutCenterTop => tiles.outer_top_center(),
        TileType::BankOutRightTop => tiles.outer_top_right(),
    };

    Sprite::from_atlas_image(image.clone(), texture_atlas)
}

fn add_collider(mut entity: EntityCommands, tile_type: TileType) {
    let maybe_isometry = match tile_type {
        TileType::Water => None,
        TileType::BankInLeftBottom => Some((vec2(109.0, 110.0), vec2(10.0, 10.0))),
        TileType::BankInRightBottom => Some((vec2(109.0, 110.0), vec2(-10.0, 10.0))),
        TileType::BankOutLeftMiddle => Some((vec2(100.0, 128.0), vec2(-15.0, 0.0))),
        TileType::BankOutRightMiddle => Some((vec2(100.0, 128.0), vec2(15.0, 0.0))),
        TileType::BankInLeftMiddle => Some((vec2(109.0, 128.0), vec2(10.0, 0.0))),
        TileType::BankInRightMiddle => Some((vec2(109.0, 128.0), vec2(-10.0, 0.0))),
        TileType::BankInLeftTop => Some((vec2(109.0, 110.0), vec2(10.0, -10.0))),
        TileType::BankInRightTop => Some((vec2(109.0, 110.0), vec2(-10.0, -10.0))),
        TileType::BankOutLeftTop => Some((vec2(109.0, 109.0), vec2(0.0, 0.0))), // TODO: adjust size
        TileType::BankOutCenterTop => Some((vec2(109.0, 109.0), vec2(0.0, 0.0))), // TODO: adjust size
        TileType::BankOutRightTop => Some((vec2(109.0, 109.0), vec2(0.0, 0.0))), // TODO: adjust size
    };

    if let Some((sizes, offset)) = maybe_isometry {
        entity.insert(Collider::new(sizes, offset));
    }
}

fn despawn_level(
    mut commands: Commands,
    tiles: Query<Entity, With<RiverBank>>,
) {
    for tile in tiles {
        commands.entity(tile).despawn();
    }
}
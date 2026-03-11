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

fn add_collider(mut entity: EntityCommands, tile_type: TileType) {
    let maybe_isometry = match tile_type {
        TileType::Water => None,
        TileType::BankLeftBottom => Some((vec2(109.0, 110.0), vec2(10.0, 10.0))),
        TileType::BankRightBottom => Some((vec2(109.0, 110.0), vec2(-10.0, 10.0))),
        TileType::BankOutLeftMiddle => Some((vec2(100.0, 128.0), vec2(-15.0, 0.0))),
        TileType::BankOutRightMiddle => Some((vec2(100.0, 128.0), vec2(15.0, 0.0))),
        TileType::BankInLeftMiddle => Some((vec2(109.0, 128.0), vec2(10.0, 0.0))),
        TileType::BankInRightMiddle => Some((vec2(109.0, 128.0), vec2(-10.0, 0.0))),
        TileType::BankLeftTop => Some((vec2(109.0, 110.0), vec2(10.0, -10.0))),
        TileType::BankRightTop => Some((vec2(109.0, 110.0), vec2(-10.0, -10.0))),
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
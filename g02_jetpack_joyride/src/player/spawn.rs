use crate::animation::Animator;
use crate::player::Player;
use crate::prelude::*;
use crate::utils;
use crate::weapon::{Muzzle, Weapon};

pub const FUCHSIA: Srgba = Srgba::rgb(1.0, 0.0, 1.0);

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load(asset_path::PLAYER_IMAGE);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(25), 4, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    let animator = Animator::new(7.0, 0, 1);

    commands.spawn((
        utils::new_name("Player"),
        Player,
        ( // view
          Sprite::from_atlas_image(
              image,
              TextureAtlas {
                  layout: layout_handle,
                  index: animator.first_index(),
              },
          ),
          animator,
          Transform {
              translation: vec3(constants::PLAYER_X, constants::GROUND_Y, z_order::PLAYER),
              rotation: Quat::default(),
              scale: Vec3::splat(1.0),
          },
        ),
    ));
}

pub fn add_weapon_to_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    players: Query<(Entity), With<Player>>,
) {
    let weapon_image = asset_server.load(asset_path::WEAPON_IMAGE);

    for player in players {
        let weapon = commands.spawn((
            utils::new_name("Weapon"),
            Weapon,
            Sprite {
                image: weapon_image.clone(),
                ..default()
            },
            Transform {
                translation: vec3(-5.5, -6.2, 1.0),
                rotation: utils::rotation_2d(256.0),
                scale: Vec3::splat(0.75),
            },
            ChildOf(player),
        )).id();

        commands.spawn((
            utils::new_name("Muzzle"),
            Muzzle,
            ChildOf(weapon),
            Transform::from_xyz(8.5, 2.1, 0.0),
        ));
    }
}

pub fn debug_muzzle_position(
    mut gizmos: Gizmos,
    muzzles: Query<&GlobalTransform, With<Muzzle>>,
) {
    for transform in muzzles {
        let world_position = transform.translation();
        gizmos.cross_2d(world_position.truncate(), 5.0, FUCHSIA);
    }
}
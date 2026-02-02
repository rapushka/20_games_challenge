use crate::player::{Player, FUCHSIA};
use crate::prelude::*;
use crate::utils;
use crate::weapon::{HeldMuzzle, Muzzle, ShootTimer, Weapon};

pub fn add_weapon_to_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    players: Query<Entity, With<Player>>,
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

        let muzzle = commands.spawn((
            utils::new_name("Muzzle"),
            Muzzle,
            ChildOf(weapon),
            Transform::from_xyz(8.5, 2.1, 0.0),
        )).id();

        commands.entity(player)
            .insert(HeldMuzzle(muzzle))
            .insert(ShootTimer::new())
        ;
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
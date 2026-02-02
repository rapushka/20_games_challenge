use crate::player::Player;
use crate::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn((
        Player,
        Sprite {
            color: crate::utils::color_from("#8888ff"),
            custom_size: Some(vec2(50.0, 125.0)),
            ..default()
        },
        Transform::from_xyz(constants::PLAYER_X, constants::GROUND_Y, z_order::PLAYER),
    ));
}
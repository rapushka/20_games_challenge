use crate::camera::CameraFollowYTarget;
use crate::player::movement::ScrollSpeedMultiplier;
use crate::position::{WorldPosition, ZOrder};
use crate::prelude::*;

pub mod plugin;
mod movement;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        CameraFollowYTarget::new(250.0),
        ScrollSpeedMultiplier::default(),
        Sprite {
            color: utils::from_hex("#ffffff"),
            custom_size: Some(vec2(100.0, 150.0)),
            ..default()
        },
        WorldPosition::ZERO,
        ZOrder::Player,
    ));
}
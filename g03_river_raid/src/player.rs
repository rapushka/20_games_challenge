use crate::camera::CameraFollowYTarget;
use crate::collision_detection::Collider;
use crate::player::movement::ScrollSpeedMultiplier;
use crate::position::{WorldPosition, ZOrder};
use crate::prelude::*;

pub mod plugin;
mod movement;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let image = asset_server.load(asset_path::PLAYER);

    commands.spawn((
        Name::new("Player"),
        Player,
        CameraFollowYTarget::new(250.0),
        ScrollSpeedMultiplier::default(),
        Sprite::from_image(image),
        WorldPosition::ZERO,
        ZOrder::Player,
        Collider::new(vec2(25.0, 85.0), vec2(0.0, -10.0)),
    ));
}
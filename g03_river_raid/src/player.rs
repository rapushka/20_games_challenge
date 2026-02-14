use crate::prelude::*;

pub mod plugin;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        Sprite {
            color: utils::from_hex("#ffffff"),
            custom_size: Some(vec2(100.0, 250.0)),
            ..default()
        },
        Transform::default(),
    ));
}
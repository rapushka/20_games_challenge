use bevy::prelude::*;

pub mod z_order {
    pub const BACKGROUND: f32 = 0.0;
    pub const PLAYER: f32 = 10.0;
}

const PLAYER_X: f32 = -450.0;
const GROUND_Y: f32 = -250.0;

#[derive(Component)]
struct Player;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            spawn_camera,
            spawn_player,
        ).chain())
        .run()
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}

fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn((
        Player,
        Sprite {
            color: color_from("#8888ff"),
            custom_size: Some(vec2(50.0, 125.0)),
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_Y, z_order::PLAYER),
    ));
}

fn color_from(hex: &'static str) -> Color {
    Srgba::hex(hex).unwrap().into()
}
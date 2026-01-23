use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub mod z_order {
    pub const DIVIDER: f32 = 1.0;
    pub const WALLS: f32 = 2.0;
    pub const PADS: f32 = 2.0;
    pub const BALL: f32 = 4.0;
}

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))

        .add_systems(Startup, (
            spawn_camera,
            create_level,
        ).chain())
        .run()
}

#[derive(Component)]
struct Collider(pub Vec2);

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}

fn create_level(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single().unwrap();
    let half_height = window.resolution.height() * 0.5;
    spawn_wall(&mut commands, half_height);
    spawn_wall(&mut commands, -half_height);
}

fn spawn_wall(
    commands: &mut Commands,
    y_position: f32,
) {
    let sizes = vec2(10_000.0, 50.0);
    commands.spawn((
        Collider(sizes),
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.9),
            custom_size: Some(sizes),
            ..default()
        },
        Transform::from_xyz(0.0, y_position, z_order::WALLS)
    ));
}
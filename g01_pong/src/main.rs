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
    spawn_wall(&mut commands, half_height + 10.0);
    spawn_wall(&mut commands, -half_height - 10.0);

    spawn_divider(
        &mut commands,
        vec2(0.0, half_height),
        vec2(0.0, -half_height),
        Color::srgb(0.45, 0.45, 0.45),
    );
}

fn spawn_wall(
    commands: &mut Commands,
    y_position: f32,
) {
    let sizes = vec2(10_000.0, 25.0);
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

fn spawn_divider(
    commands: &mut Commands,
    start: Vec2,
    end: Vec2,
    color: Color,
) {
    const DASH_LENGTH: f32 = 25.0;
    const GAP_LENGTH: f32 = 25.0;
    const THICKNESS: f32 = 5.0;

    let delta = end - start;
    let length = delta.length();
    let direction = delta.normalize();

    let mut distance = 0.0;

    while distance < length {
        let segment_start = start + direction * distance;
        let segment_end = start + direction * (distance + DASH_LENGTH).min(length);

        let segment_length = (segment_end - segment_start).length();
        let center = (segment_start + segment_end) * 0.5;

        commands.spawn((
            Sprite {
                color,
                custom_size: Some(vec2(segment_length, THICKNESS)),
                ..default()
            },
            Transform {
                translation: center.extend(z_order::DIVIDER),
                rotation: Quat::from_rotation_z(direction.y.atan2(direction.x)),
                ..default()
            },
        ));

        distance += DASH_LENGTH + GAP_LENGTH;
    }
}
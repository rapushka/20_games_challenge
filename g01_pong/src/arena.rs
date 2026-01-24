use bevy::math::bounding::Aabb2d;
use crate::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_level(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single().unwrap();
    let half_height = window.resolution.height() * 0.5 + 6.0;
    spawn_wall(&mut commands, half_height);
    spawn_wall(&mut commands, -half_height);

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
    let transform = Transform::from_xyz(0.0, y_position, z_order::WALL);

    commands.spawn((
        Collider(Aabb2d::new(
            transform.translation.truncate(),
            sizes / 2.0,
        )),
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.9),
            custom_size: Some(sizes),
            ..default()
        },
        transform,
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
use bevy::window::PrimaryWindow;
use crate::paddle::{YBounds, Paddle, Side, PADDLE_SIZE};
use crate::prelude::*;

pub fn spawn_paddles(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single().unwrap();
    let half_width = window.resolution.width() * 0.5 - 100.0;
    let half_height = window.resolution.height() * 0.5 - 100.0;

    let bounds = YBounds { min: -half_height, max: half_height };

    create_paddle(&mut commands, bounds, -half_width, Side::Left);
    create_paddle(&mut commands, bounds, half_width, Side::Right);
}

fn create_paddle(
    commands: &mut Commands,
    y_bounds: YBounds,
    x_position: f32,
    side: Side,
) {
    commands.spawn((
        Paddle,
        y_bounds,
        side,
        Collider(PADDLE_SIZE),
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.9),
            custom_size: Some(PADDLE_SIZE),
            ..default()
        },
        Transform::from_xyz(x_position, 0.0, z_order::PADDLE),
    ));
}
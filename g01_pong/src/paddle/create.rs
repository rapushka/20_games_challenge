use crate::paddle::{Paddle, Side, PADDLE_SIZE};
use crate::prelude::*;
use bevy::window::PrimaryWindow;
use crate::bounds::Bounds;

pub fn spawn_paddles(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single().unwrap();
    let half_width = window.resolution.width() * 0.5 - 100.0;
    let half_height = window.resolution.height() * 0.5 - 70.0;

    let bounds = Bounds::new_square_y(half_height);

    create_paddle(&mut commands, bounds, -half_width, Side::Left);
    create_paddle(&mut commands, bounds, half_width, Side::Right);
}

fn create_paddle(
    commands: &mut Commands,
    y_bounds: Bounds,
    x_position: f32,
    side: Side,
) {
    let sizes = PADDLE_SIZE;
    let transform = Transform::from_xyz(x_position, 0.0, z_order::PADDLE);

    commands.spawn((
        Paddle,
        y_bounds,
        side,
        Collider::new(transform.translation.truncate(), sizes),
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.9),
            custom_size: Some(sizes),
            ..default()
        },
        transform,
    ));
}
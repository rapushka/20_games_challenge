use crate::prelude::*;

pub use create::*;
use crate::bounds::Bounds;
use crate::input::Movement;

mod create;

const PADDLE_SIZE: Vec2 = Vec2::new(25.0, 125.0);
const PADDLE_MOVEMENT_SPEED: f32 = 400.0;

#[derive(Component)]
pub struct Paddle;

#[derive(Component, Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
}

pub fn move_paddles(
    paddles: Query<(&mut Transform, &Movement, &Bounds), With<Paddle>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (mut transform, movement, y_bounds) in paddles {
        let y = transform.translation.y + (movement.y() * PADDLE_MOVEMENT_SPEED * delta_time);
        transform.translation.y = y_bounds.clamp_y(y);
    }
}
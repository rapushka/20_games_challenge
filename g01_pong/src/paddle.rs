use crate::prelude::*;

pub use create::*;
mod create;

const PADDLE_SIZE: Vec2 = Vec2::new(25.0, 125.0);

#[derive(Component)]
pub struct Paddle;

#[derive(Component, Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
}
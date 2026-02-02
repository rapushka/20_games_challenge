use crate::prelude::*;

// -> Player
#[derive(Message)]
pub struct Shoot(pub Entity);

#[derive(Component)]
pub struct Bullet;
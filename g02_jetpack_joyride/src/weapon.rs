use crate::prelude::*;

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct Muzzle;

// Player -> Muzzle
#[derive(Component)]
pub struct HeldMuzzle(pub Entity);
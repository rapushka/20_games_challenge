use crate::prelude::*;
pub use spawn::*;
pub use movement::*;
pub use death::*;

mod spawn;
mod movement;
mod death;

#[derive(Component)]
pub struct Player;
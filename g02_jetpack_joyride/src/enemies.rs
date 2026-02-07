use crate::prelude::*;

pub use spawn_timer::*;
pub use spawn::*;
pub use enemy_type::*;

mod spawn_timer;
mod spawn;
mod enemy_type;

#[derive(Component)]
pub struct Enemy;
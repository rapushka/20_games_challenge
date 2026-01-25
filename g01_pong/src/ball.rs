use crate::prelude::*;
use rand::prelude::*;

pub use create::*;
pub use movement::*;
pub use scoring::*;

mod create;
mod movement;
mod scoring;

#[derive(Component)]
pub struct Ball;
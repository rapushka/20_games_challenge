use crate::prelude::*;

#[derive(Resource, Reflect, Clone, Eq, PartialEq, Debug, Default)]
pub struct Highscore(u32);
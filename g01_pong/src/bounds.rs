use crate::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct YBounds {
    min: f32,
    max: f32,
}

impl YBounds {
    pub fn new_square(scalar: f32) -> Self {
        YBounds {
            min: -scalar,
            max: scalar,
        }
    }

    pub fn clamp(&self, source: f32) -> f32 {
        source.clamp(self.min, self.max)
    }
}
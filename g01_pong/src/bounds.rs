use crate::prelude::*;

#[derive(Component, Copy, Clone)]
pub struct Bounds {
    min: Vec2,
    max: Vec2,
}

impl Bounds {
    pub fn new_square_y(scalar: f32) -> Self {
        Bounds {
            min: vec2(0.0, -scalar),
            max: vec2(0.0, scalar),
        }
    }

    pub fn new_square_x(scalar: f32) -> Self {
        Bounds {
            min: vec2(-scalar, 0.0),
            max: vec2(scalar, 0.0),
        }
    }

    pub fn clamp_y(&self, source: f32) -> f32 {
        source.clamp(self.min.y, self.max.y)
    }

    pub fn inside_x(&self, value: f32) -> bool {
        self.min.x < value && self.max.x > value
    }
}
use bevy::math::bounding::Aabb2d;
use crate::prelude::*;

#[derive(Component)]
pub struct Collider {
    half_size: Vec2,
    center: Vec2,
}

impl Collider {
    pub fn new(center: Vec2, sizes: Vec2) -> Collider {
        Collider {
            half_size: sizes / 2.0,
            center,
        }
    }

    pub fn set_center(&mut self, value: Vec2) {
        self.center = value;
    }

    pub fn aabb(&self) -> Aabb2d {
        Aabb2d::new(self.center, self.half_size)
    }
}

pub fn update_colliders(
    colliders: Query<(&mut Collider, &Transform)>
) {
    for (mut collider, transform) in colliders {
        collider.set_center(transform.translation.truncate());
    }
}

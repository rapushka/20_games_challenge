use bevy::math::FloatPow;
use crate::player::FUCHSIA;
use crate::prelude::*;

#[derive(Component)]
pub struct Collider {
    radius: f32,
    center: Vec2,
}

impl Collider {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self { radius, center }
    }

    pub fn set_center(&mut self, value: Vec2) {
        self.center = value;
    }

    pub fn is_collides(&self, other: &Self) -> bool {
        let radii_squared = (self.radius + other.radius).squared();
        let distance_squared = self.center.distance_squared(other.center);

        distance_squared < radii_squared
    }
}

pub fn update_colliders(
    colliders: Query<(&mut Collider, &Transform)>
) {
    for (mut collider, transform) in colliders {
        collider.set_center(transform.translation.truncate());
    }
}

pub fn debug_colliders(
    mut gizmos: Gizmos,
    colliders: Query<&Collider>,
) {
    for collider in colliders {
        gizmos.circle_2d(collider.center, collider.radius, FUCHSIA);
    }
}

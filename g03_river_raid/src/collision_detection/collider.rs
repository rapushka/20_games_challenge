use bevy::math::bounding::Aabb2d;
use crate::prelude::{Component, Gizmos, Query, Srgba, Transform, Vec2};

pub const FUCHSIA: Srgba = Srgba::rgb(1.0, 0.0, 1.0);

#[derive(Component)]
pub struct Collider {
    half_size: Vec2,
    center: Vec2,
    offset: Vec2,
}

impl Collider {
    pub fn new(sizes: Vec2, offset: Vec2) -> Collider {
        Collider {
            half_size: sizes / 2.0,
            offset,
            center: Vec2::ZERO,
        }
    }

    pub fn size(&self) -> Vec2 {
        self.half_size * 2.0
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

pub fn debug_colliders(
    mut gizmos: Gizmos,
    colliders: Query<&Collider>,
) {
    for collider in colliders {
        gizmos.rect_2d(collider.center, collider.size(), FUCHSIA);
    }
}
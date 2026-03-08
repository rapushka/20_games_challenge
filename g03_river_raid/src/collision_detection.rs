use crate::prelude::*;

pub use collider::*;
mod collider;

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_colliders,
            ))
        ;
    }
}
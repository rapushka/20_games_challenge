use crate::prelude::*;

pub use collider::*;
use crate::order::{FixedUpdateOrder, UpdateOrder};

mod collider;

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, (
                update_colliders,
            ).in_set(FixedUpdateOrder::UpdateColliderPosition))
        ;
    }
}
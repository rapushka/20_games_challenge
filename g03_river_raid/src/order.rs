use crate::prelude::*;

pub struct SystemOrderPlugin;

impl Plugin for SystemOrderPlugin {
    fn build(&self, app: &mut App) {
        app
            .configure_sets(Update, (
                UpdateOrder::Input,
                UpdateOrder::Camera,
                UpdateOrder::UpdateTransforms,
                UpdateOrder::Debug,
                UpdateOrder::Cleanups,
            ).chain())

            .configure_sets(FixedUpdate, (
                FixedUpdateOrder::Movement,
                FixedUpdateOrder::UpdateColliderPosition,
                FixedUpdateOrder::CollisionDetection,
                FixedUpdateOrder::HandleCollisions,
                FixedUpdateOrder::ReactiveGameLogic,
            ).chain())
        ;
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FixedUpdateOrder {
    Movement,
    UpdateColliderPosition,
    CollisionDetection,
    HandleCollisions,

    ReactiveGameLogic,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateOrder {
    Input,
    Camera,
    UpdateTransforms,
    Debug,
    Cleanups,
}
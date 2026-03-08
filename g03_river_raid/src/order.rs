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
            ).chain())

            .configure_sets(FixedUpdate, (
                FixedUpdateOrder::GameLogic,
                FixedUpdateOrder::UpdateColliderPosition,
                FixedUpdateOrder::CollisionDetection,
            ).chain())
        ;
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FixedUpdateOrder {
    GameLogic,
    UpdateColliderPosition,
    CollisionDetection,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateOrder {
    Input,
    Camera,
    UpdateTransforms,
    Debug,
}
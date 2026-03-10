use crate::prelude::*;
use crate::level::RiverBank;
use crate::order::*;
use crate::player::Player;

pub use collider::*;
pub use collision::*;

mod collider;
mod collision;

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<Collision>()

            .add_systems(FixedUpdate, (
                update_colliders,
            ).in_set(FixedUpdateOrder::UpdateColliderPosition))

            .add_systems(FixedUpdate, (
                check_collisions::<Player, RiverBank>,
            ).in_set(FixedUpdateOrder::CollisionDetection))
        ;
    }
}

fn check_collisions<TSubject: Component, TObject: Component>(
    mut collision_messages: MessageWriter<Collision>,
    subjects: Query<(Entity, &Collider), With<TSubject>>,
    objects: Query<(Entity, &Collider), With<TObject>>,
) {
    for (subject, subject_collider) in subjects {
        for (object, object_collider) in objects {
            if subject == object {
                continue;
            }

            if subject_collider.is_colliding(object_collider) {
                collision_messages.write(Collision::new(subject, object));
            }
        }
    }
}
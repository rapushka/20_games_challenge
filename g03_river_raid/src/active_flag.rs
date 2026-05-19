use crate::order::FixedUpdateOrder;
use crate::player::Player;
use crate::position::WorldPosition;
use crate::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
// To mark objects Inactive when they're offscreen
pub struct Active;

#[derive(Component, Default)]
// Marks that entity can be Active/Inactive
pub struct DynamicObject;

pub struct ActiveFlagPlugin;

impl Plugin for ActiveFlagPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, (
                update_active_flag,
            ).in_set(FixedUpdateOrder::UpdateActiveFlag))
        ;
    }
}

fn update_active_flag(
    mut commands: Commands,
    dynamic_objects: Query<(Entity, &WorldPosition), With<DynamicObject>>,
    players: Query<&WorldPosition, With<Player>>,
) {
    for player_position in players {
        for (object_entity, object_position) in dynamic_objects {
            let distance = object_position.y - player_position.y;
            let max_distance = constants::ACTIVE_OBJECTS_DISTANCE;

            if distance < max_distance {
                commands.entity(object_entity).insert(Active);
            } else {
                commands.entity(object_entity).remove::<Active>();
            }
        }
    }
}
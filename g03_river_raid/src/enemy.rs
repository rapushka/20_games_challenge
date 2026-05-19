use crate::active_flag::DynamicObject;
use crate::order::FixedUpdateOrder;
use crate::prelude::*;

#[derive(Component)]
#[require(DynamicObject)]
pub struct Enemy;

mod jet_enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Initialize), (
                jet_enemy::spawn_jets_on_level,
            ))

            .add_systems(FixedUpdate, (
                jet_enemy::move_jets,
            ).in_set(FixedUpdateOrder::Movement))

            .add_systems(FixedUpdate, (
                jet_enemy::despawn_offscreen_jets,
            ).in_set(FixedUpdateOrder::ReactiveGameLogic))

            .add_systems(OnEnter(AppState::Restarting), (
                despawn_all_enemies,
            ))
        ;
    }
}

fn despawn_all_enemies(
    mut commands: Commands,
    tiles: Query<Entity, With<Enemy>>,
) {
    for tile in tiles {
        commands.entity(tile).despawn();
    }
}
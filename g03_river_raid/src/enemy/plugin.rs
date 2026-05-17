use crate::prelude::*;

#[derive(Component)]
pub struct Enemy;

mod jet_enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Initialize), (
                jet_enemy::spawn_jets_on_level,
            ))
        ;
    }
}
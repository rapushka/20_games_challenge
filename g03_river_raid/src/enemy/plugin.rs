use crate::enemy::plugin::jet_enemy::JetEnemyPlugin;
use crate::prelude::*;

#[derive(Component)]
pub struct Enemy;

mod jet_enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(JetEnemyPlugin)
        ;
    }
}
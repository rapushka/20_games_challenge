use crate::prelude::*;

pub use spawn::*;
pub use movement::*;
use crate::collision_detection::PlayerTouchedEnemy;
use crate::game::IsGameStarted;

mod spawn;
mod movement;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Dead;

pub fn player_die(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    mut message: MessageReader<PlayerTouchedEnemy>,
    mut is_game_started: ResMut<IsGameStarted>,
) {
    for _ in message.read() {
        for player in players {
            commands.entity(player).insert(Dead);
        }

        is_game_started.stop();
    }
}
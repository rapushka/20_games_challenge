use crate::prelude::*;

pub use spawn::*;
pub use movement::*;
use crate::collision_detection::PlayerTouchedEnemy;
use crate::game::IsGameStarted;

mod spawn;
mod movement;

#[derive(Component)]
pub struct Player;

pub fn player_die(
    mut message: MessageReader<PlayerTouchedEnemy>,
    mut is_game_started: ResMut<IsGameStarted>,
) {
    for _ in message.read() {
        info!("PLAYER DED");
        is_game_started.stop();
    }
}
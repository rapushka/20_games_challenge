use crate::prelude::*;

pub use collider::*;
use crate::enemies::Enemy;
use crate::game::IsGameStarted;
use crate::player::Player;

mod collider;

#[derive(Message)]
pub struct PlayerTouchedEnemy;

pub fn check_collisions(
    mut message: MessageWriter<PlayerTouchedEnemy>,
    players: Query<&Collider, With<Player>>,
    enemies: Query<&Collider, With<Enemy>>,
    is_game_started: Res<IsGameStarted>,
) {
    if !is_game_started.is_started() {
        return;
    }

    for player_collider in players {
        for enemy_collider in enemies {
            if player_collider.is_collides(enemy_collider) {
                message.write(PlayerTouchedEnemy);
            }
        }
    }
}
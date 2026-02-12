use crate::prelude::*;

pub use collider::*;
use crate::bullet::Bullet;
use crate::death::Dead;
use crate::enemies::Enemy;
use crate::game::IsGameStarted;
use crate::player::Player;

mod collider;

#[derive(Message)]
pub struct PlayerTouchedEnemy;

pub fn collide_player_and_enemy(
    mut message: MessageWriter<PlayerTouchedEnemy>,
    players: Query<&Collider, (With<Player>, Without<Dead>)>,
    enemies: Query<&Collider, (With<Enemy>, Without<Dead>)>,
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

pub fn collide_enemy_and_bullet(
    mut commands: Commands,
    enemies: Query<(Entity, &Collider), (With<Enemy>, Without<Dead>)>,
    bullets: Query<&Collider, With<Bullet>>,
    is_game_started: Res<IsGameStarted>,
) {
    if !is_game_started.is_started() {
        return;
    }

    for (enemy, enemy_collider) in enemies {
        for bullet_collider in bullets {
            if enemy_collider.is_collides(bullet_collider) {
                commands.entity(enemy)
                    .insert(Dead);
            }
        }
    }
}
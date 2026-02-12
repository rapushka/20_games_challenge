use crate::collision_detection::PlayerTouchedEnemy;
use crate::death::Dead;
use crate::game::IsGameStarted;
use crate::player::Player;
use crate::prelude::*;

pub fn player_die(
    mut commands: Commands,
    mut players: Query<Entity, With<Player>>,
    mut message: MessageReader<PlayerTouchedEnemy>,
    mut is_game_started: ResMut<IsGameStarted>,
) {
    for _ in message.read() {
        for player in &mut players {
            commands.entity(player)
                // .insert(Animator::one_frame(3))
                .insert(Dead);
        }

        is_game_started.stop();
    }
}
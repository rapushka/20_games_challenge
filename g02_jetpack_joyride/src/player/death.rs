use crate::audio_player::PlaySoundCommandExtensions;
use crate::collision_detection::PlayerTouchedEnemy;
use crate::death::Dead;
use crate::game::IsGameStarted;
use crate::player::Player;
use crate::prelude::*;

pub fn on_player_touched_enemy(
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

pub fn play_sound_on_player_dead(
    mut commands: Commands,
    mut message_reader: MessageReader<PlayerTouchedEnemy>,
) {
    for _ in message_reader.read() {
        commands.play_sound_with_volume(asset_path::LOOSE_SOUND, 0.25);
    }
}
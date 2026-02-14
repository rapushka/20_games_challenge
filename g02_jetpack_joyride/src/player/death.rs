use bevy::audio::Volume;
use crate::bullet::BulletHit;
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
    asset_server: Res<AssetServer>,
) {
    let sound = asset_server.load(asset_path::LOOSE_SOUND);

    for _ in message_reader.read() {
        commands.spawn((
            AudioPlayer::new(sound.clone()),
            PlaybackSettings {
                volume: Volume::Linear(0.25),
                ..PlaybackSettings::DESPAWN
            },
        ));
    }
}
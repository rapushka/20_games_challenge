use bevy::audio::Volume;
use crate::audio_player::PlaySoundCommand;
use crate::prelude::*;

pub trait PlaySoundCommandExtensions {
    fn play_sound(&mut self, path: &'static str);

    fn play_sound_with_volume(&mut self, path: &'static str, volume: f32);
}

impl<'w, 's> PlaySoundCommandExtensions for Commands<'w, 's> {
    fn play_sound(&mut self, path: &'static str) {
        self.queue(PlaySoundCommand {
            path,
            settings: PlaybackSettings::DESPAWN,
        });
    }

    fn play_sound_with_volume(&mut self, path: &'static str, volume: f32) {
        self.queue(PlaySoundCommand {
            path,
            settings: PlaybackSettings {
                volume: Volume::Linear(volume),
                ..PlaybackSettings::DESPAWN
            },
        });
    }
}
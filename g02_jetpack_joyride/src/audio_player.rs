use crate::prelude::*;
use bevy::asset::io::embedded::GetAssetServer;

pub use extensions::*;
mod extensions;

struct PlaySoundCommand {
    path: &'static str,
    settings: PlaybackSettings,
}

impl Command for PlaySoundCommand {
    fn apply(self, world: &mut World) {
        let handle = world.get_asset_server().load(self.path);

        world.spawn((
            AudioPlayer::new(handle),
            self.settings,
        ));
    }
}
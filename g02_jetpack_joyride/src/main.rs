use crate::animation::*;
use crate::background::{scroll_background, spawn_background};
use crate::player::*;
use crate::prelude::*;

pub mod z_order {
    pub const BACKGROUND: f32 = 0.0;
    pub const PLAYER: f32 = 10.0;
}

pub mod constants {
    use crate::prelude::*;

    pub const PLAYER_X: f32 = -450.0;

    pub const GROUND_Y: f32 = MIN_Y;
    pub const MIN_Y: f32 = -250.0;

    pub const MAX_Y: f32 = 250.0;
    pub const ASCENDING_SPEED: f32 = 250.0;

    pub const DESCENDING_SPEED: f32 = 250.0;

    pub const BUTTON: KeyCode = KeyCode::Space;

    pub const LEVEL_SCROLL_SPEED: f32 = 250.0;
}

pub mod asset_path {
    pub const PLAYER_IMAGE: &str = "player/tilemap.png";

    pub const BG_IMAGE: &str = "environment/bg.png";
}

pub mod prelude;
mod utils;
mod animation;
mod player;
mod background;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
        )

        .add_systems(Startup, (
            spawn_camera,
            spawn_player,
            spawn_background,
        ).chain())

        .add_systems(Update, (
            ascend_player,
            update_is_ascending,
            descent_player,

            // view
            scroll_background,
            animate_sprites,
        ).chain())

        .run()
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}
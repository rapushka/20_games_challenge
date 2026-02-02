use bevy::camera::ScalingMode;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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

    // player
    pub const PLAYER_X: f32 = -95.0;
    pub const ASCENDING_SPEED: f32 = 250.0;
    pub const DESCENDING_SPEED: f32 = 250.0;
    pub const BUTTON: KeyCode = KeyCode::Space;

    pub const GROUND_Y: f32 = MIN_Y;

    pub const MIN_Y: f32 = -52.5;
    pub const MAX_Y: f32 = 50.0;

    pub const LEVEL_SCROLL_SPEED: f32 = 75.0;
}

pub mod asset_path {
    pub const PLAYER_IMAGE: &str = "player/tilemap.png";

    pub const BG_IMAGE: &str = "environment/bg-scaled.png";

    pub const WEAPON_IMAGE: &str = "weapon/riffle.png";
}

pub mod prelude;
mod utils;
mod animation;
mod player;
mod weapon;
mod background;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest()),

            // egui
            EguiPlugin::default(),
            WorldInspectorPlugin::new()
        ))

        .add_systems(Startup, (
            spawn_camera,
            spawn_player,
            spawn_background,
            add_weapon_to_players,
        ).chain())

        .add_systems(Update, (
            ascend_player,
            update_is_ascending,
            descent_player,

            // view
            scroll_background,
            animate_sprites,

            //debug
            #[cfg(debug_assertions)]
            debug_muzzle_position,
        ).chain())

        .run()
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: 240.0,
                max_height: 135.0,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
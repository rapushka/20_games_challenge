use bevy::camera::ScalingMode;
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::weapon::*;
use crate::animation::*;
use crate::background::*;
use crate::bullet::*;
use crate::player::*;
use crate::prelude::*;

pub mod z_order {
    pub const BACKGROUND: f32 = 0.0;
    pub const PLAYER: f32 = 10.0;
}

pub mod constants {
    use crate::prelude::*;

    // player
    pub const PLAYER_X: f32 = -90.0;
    pub const ASCENDING_SPEED: f32 = 200.0;
    pub const DESCENDING_SPEED: f32 = 150.0;

    // input
    pub const ASCEND_BUTTON: KeyCode = KeyCode::Space;
    pub const TOGGLE_DEBUG_MODE_BUTTON: KeyCode = KeyCode::Backquote;

    pub const GROUND_Y: f32 = MIN_Y;

    pub const MIN_Y: f32 = -51.0;
    pub const MAX_Y: f32 = 50.0;

    pub const LEVEL_SCROLL_SPEED: f32 = 75.0;

    // bullets
    pub const SHOOT_BULLET_DELAY_S: f32 = 0.02;
    pub const BULLET_FLY_SPEED: f32 = 500.0;
    pub const BULLET_DESPAWN_Y: f32 = GROUND_Y - 8.5;
    pub const BULLET_MAX_SPREAD: f32 = 10.0;
    pub const BULLET_COLOR: &str = "#fce571";
    pub const BULLET_SIZE: Vec2 = vec2(7.0, 1.5);
}

pub mod asset_path {
    pub const PLAYER_IMAGE: &str = "player/tilemap.png";

    pub const BG_IMAGE: &str = "environment/bg_v4.png";

    pub const WEAPON_IMAGE: &str = "weapon/riffle.png";
}

pub mod prelude;
mod utils;
mod animation;
mod player;
mod weapon;
mod bullet;
mod background;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest()),

            // egui
            EguiPlugin::default(),
            WorldInspectorPlugin::new()
                .run_if(input_toggle_active(false, constants::TOGGLE_DEBUG_MODE_BUTTON)),
        ))

        .add_message::<Shoot>()

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
            tick_shooting_timer_while_ascending,
            shoot_bullets,
            fly_bullets,
            despawn_hit_bullets,

            // view
            scroll_background,
            animate_sprites,

            //debug
            #[cfg(debug_assertions)]
            (
                debug_muzzle_position,
            ).run_if(input_toggle_active(false, constants::TOGGLE_DEBUG_MODE_BUTTON)),
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
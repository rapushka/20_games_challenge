use bevy::camera::ScalingMode;
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use score::*;
use crate::weapon::*;
use crate::animation::*;
use crate::background::*;
use crate::bullet::*;
use crate::collision_detection::*;
use crate::enemies::*;
use crate::game::*;
use crate::player::*;
use crate::prelude::*;

pub mod z_order {
    pub const BACKGROUND: f32 = 0.0;
    pub const PLAYER: f32 = 10.0;
    pub const ENEMIES: f32 = 9.0;
}

pub mod constants {
    use crate::prelude::*;

    pub const CANVAS_SIZE: Vec2 = vec2(240.0, 135.0);
    pub const CANVAS_HALF_SIZE: Vec2 = vec2(CANVAS_SIZE.x * 0.5, CANVAS_SIZE.y * 0.5);

    // player
    pub const PLAYER_X: f32 = -90.0;
    pub const ASCENDING_SPEED: f32 = 200.0;
    pub const DESCENDING_SPEED: f32 = 150.0;

    // enemy
    pub const ENEMY_SPAWN_X: f32 = CANVAS_HALF_SIZE.x + 50.0;
    pub const ENEMY_MOVEMENT_SPEED: f32 = LEVEL_SCROLL_SPEED;
    pub const ENEMY_DESPAWN_X: f32 = -CANVAS_HALF_SIZE.x - 50.0;

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
    pub const ENEMIES_IMAGE: &str = "enemy/tilemap.png";

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
mod score;
mod game;
mod enemies;
mod collision_detection;

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

        .init_resource::<Score>()
        .init_resource::<IsGameStarted>()
        .init_resource::<EnemySpawnerTimer>()

        .add_message::<Shoot>()
        .add_message::<SpawnEnemy>()
        .add_message::<PlayerTouchedEnemy>()

        .add_systems(Startup, (
            spawn_camera,
            spawn_player,
            spawn_background,
            add_weapon_to_players,
            spawn_score_view,
        ).chain())

        .add_systems(Update, (
            // game
            start_game_on_first_click,

            // player
            ascend_player,
            update_is_ascending,
            descent_player,
            player_die,

            // enemies
            tick_enemy_spawn_timer,
            spawn_enemy,
            move_enemies,
            despawn_enemies_offscreen,

            // collision detection
            update_colliders,
            check_collisions,

            // bullets
            tick_shooting_timer_while_ascending,
            shoot_bullets,
            fly_bullets,
            despawn_hit_bullets,

            // score
            increase_score,

            // view
            scroll_background,
            animate_sprites,
            update_score_view,

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
                max_width: constants::CANVAS_SIZE.x,
                max_height: constants::CANVAS_SIZE.y,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}
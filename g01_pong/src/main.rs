use bevy::prelude::*;
use crate::ball::ResetBall;

pub mod z_order {
    pub const DIVIDER: f32 = 1.0;
    pub const WALL: f32 = 2.0;
    pub const PADDLE: f32 = 2.0;
    pub const BALL: f32 = 4.0;
}

mod prelude;
mod arena;
mod paddle;
mod collision_detection;
mod input;
mod ball;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))

        .add_message::<ResetBall>()

        .add_systems(Startup, (
            spawn_camera,
            arena::spawn_level,
            paddle::spawn_paddles,
            ball::spawn,
        ).chain())

        .add_systems(Update, (
            input::read_players_input,
            paddle::move_paddles,
            ball::update_velocity,
            ball::reset_ball,
            collision_detection::update_colliders,
            collision_detection::check_collisions,
        ).chain())

        .run()
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}
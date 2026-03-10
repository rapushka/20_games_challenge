use crate::camera::*;
use crate::collision_detection::*;
use crate::debug::*;
use crate::destroy::{Destroy, DestructionPlugin};
use crate::level::*;
use crate::order::*;
use crate::player::plugin::*;
use crate::position::*;
use crate::prelude::*;
use crate::random::*;

mod prelude;
pub mod constants;
pub mod asset_path;
pub mod utils;
mod order;
mod position;
mod app_state;
mod player;
mod camera;
mod level;
mod random;
mod debug;
mod collision_detection;
mod destroy;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            SystemOrderPlugin,
            PlayerPlugin,
            CameraPlugin,
            LevelPlugin,
            CollisionDetectionPlugin,
            DestructionPlugin,

            // debug
            #[cfg(debug_assertions)]
            DebugPlugin,
        ))
        .init_state::<AppState>()

        .init_resource::<EnvironmentTiles>()
        .init_resource::<Random>()

        // Bootstrap -> Initialize
        .add_systems(OnEnter(AppState::Bootstrap), (
            |mut next_state: ResMut<NextState<AppState>>| {
                next_state.set(AppState::Initialize);
            },
        ).chain())

        // Initialize -> Playing (tmp skip)
        .add_systems(OnEnter(AppState::Initialize), (
            |mut next_state: ResMut<NextState<AppState>>| {
                next_state.set(AppState::Playing);
            },
        ))

        // GameOver -> Initialize (tmp skip)
        .add_systems(OnEnter(AppState::GameOver), (
            |mut next_state: ResMut<NextState<AppState>>| {
                next_state.set(AppState::Initialize);
            },
        ))

        .add_systems(Update, (
            update_translations,
        ).in_set(UpdateOrder::UpdateTransforms))

        .run()
}
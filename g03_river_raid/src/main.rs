use crate::camera::*;
use crate::level::*;
use crate::player::plugin::*;
use crate::position::*;
use crate::prelude::*;
use crate::random::Random;

mod prelude;
pub mod constants;
pub mod asset_path;
pub mod utils;
mod position;
mod app_state;
mod player;
mod camera;
mod level;
mod random;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            // Third Party
            DefaultPlugins,

            // River Raid
            PlayerPlugin,
            CameraPlugin,
            LevelPlugin,
        ))
        .init_state::<AppState>()
        .init_resource::<EnvironmentTiles>()
        .init_resource::<Random>()

        .add_systems(OnEnter(AppState::Bootstrap), (
            |mut next_state: ResMut<NextState<AppState>>| {
                next_state.set(AppState::Initialize);
            },
        ).chain())

        .add_systems(OnEnter(AppState::Initialize), (
            |mut next_state: ResMut<NextState<AppState>>| {
                next_state.set(AppState::Playing);
            },
        ))

        .add_systems(PostUpdate, (
            update_translations,
        ))

        .run()
}
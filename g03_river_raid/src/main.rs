use crate::player::plugin::PlayerPlugin;
use crate::prelude::*;

mod prelude;
pub mod constants;
pub mod utils;
mod app_state;
mod player;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            // Third Party
            DefaultPlugins,

            // River Raid
            PlayerPlugin,
        ))
        .init_state::<AppState>()

        .add_systems(OnEnter(AppState::Bootstrap), (
            spawn_camera,
            proceed_to_initialize,
        ).chain())

        .run()
}

fn proceed_to_initialize(
    mut next_state: ResMut<NextState<AppState>>
) {
    next_state.set(AppState::Initialize)
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}
use crate::player::plugin::PlayerPlugin;
use crate::position::*;
use crate::prelude::*;

mod prelude;
pub mod constants;
pub mod asset_path;
pub mod utils;
mod app_state;
mod player;
mod position;

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
            tmp_spawn_background, // TODO: REMOVE
            proceed_to_initialize,
        ).chain())

        .add_systems(PostUpdate, (
            update_translations,
        ))

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
    commands.spawn((
        Camera2d,
        WorldPosition::ZERO,
    ));
}

fn tmp_spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let image = asset_server.load(asset_path::TMP_BACKGROUND);

    commands.spawn((
        Sprite {
            image,
            ..default()
        },
        WorldPosition::ZERO,
        ZOrder::Background,
    ));
}
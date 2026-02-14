use crate::prelude::*;

mod prelude;
mod app_state;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()

        .add_systems(OnEnter(AppState::Bootstrap), (
            spawn_camera,
        ))

        .run()
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}
use super::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Initialize), spawn_player)

            .add_systems(FixedUpdate, (
                move_player_x,
            ))
        ;
    }
}
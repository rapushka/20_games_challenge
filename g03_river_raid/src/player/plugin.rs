use crate::order::{FixedUpdateOrder, UpdateOrder};
use crate::player::movement::{move_player_x, player_fly_towards, update_scroll_speed_multiplier};
use super::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Initialize), spawn_player)

            .add_systems(Update, (
                update_scroll_speed_multiplier,
            ).in_set(UpdateOrder::Input))

            .add_systems(FixedUpdate, (
                move_player_x,
                player_fly_towards,
            ).in_set(FixedUpdateOrder::GameLogic))
        ;
    }
}
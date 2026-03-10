use crate::order::*;
use crate::player::death::*;
use crate::player::movement::*;
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
            ).in_set(FixedUpdateOrder::Movement))

            .add_systems(FixedUpdate, (
                on_player_collided_with_bank,
            ).in_set(FixedUpdateOrder::HandleCollisions))

            .add_systems(FixedUpdate, (
                on_player_destroyed,
            ).in_set(FixedUpdateOrder::ReactiveGameLogic))
        ;
    }
}
use crate::collision_detection::{Collision, Obstacle};
use crate::destroy::Destroy;
use crate::level::RiverBank;
use crate::player::Player;
use crate::prelude::*;

pub fn on_player_collision_with_obstacle(
    mut collisions: MessageReader<Collision>,
    mut destroy_messages: MessageWriter<Destroy>,
    players: Query<(), With<Player>>,
    obstacles: Query<(), With<Obstacle>>,
) {
    for collision in collisions.read() {
        let subject_is_player = players.contains(collision.subject());
        let object_is_obstacle = obstacles.contains(collision.object());

        if !subject_is_player || !object_is_obstacle {
            continue;
        }

        let player = collision.subject();
        destroy_messages.write(Destroy(player));
        break;
    }
}

pub fn on_player_destroyed(
    mut app_state: ResMut<NextState<AppState>>,
    mut destroy_messages: MessageReader<Destroy>,
    players: Query<(), With<Player>>,
) {
    for Destroy(entity) in destroy_messages.read() {
        let is_player = players.contains(*entity);

        if is_player {
            app_state.set(AppState::GameOver);
        }
    }
}
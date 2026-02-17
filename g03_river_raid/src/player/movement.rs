use crate::player::Player;
use crate::position::WorldPosition;
use crate::prelude::*;

pub use vertical::*;

mod vertical;

pub fn move_player_x(
    players: Query<&mut WorldPosition, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    let left = input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]);
    let right = input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]);

    for mut position in players {
        let mut direction = 0.0;

        if left {
            direction = -1.0;
        }

        if right {
            direction = 1.0;
        }

        let scaled_speed = constants::player::HORIZONTAL_SPEED * time.delta_secs();
        position.x += scaled_speed * direction;
    }
}

pub fn player_fly_towards(
    players: Query<(&mut WorldPosition, &ScrollSpeedMultiplier), With<Player>>,
    time: Res<Time<Fixed>>,
) {
    for (mut position, speed_mult) in players {
        position.y += time.delta_secs() * constants::player::FLY_SPEED * speed_mult.0;
    }
}
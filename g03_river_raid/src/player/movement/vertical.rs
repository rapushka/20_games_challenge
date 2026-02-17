use crate::prelude::*;

#[derive(Component, Default, Deref)]
pub struct ScrollSpeedMultiplier(pub f32);

pub fn update_scroll_speed_multiplier(
    entities: Query<&mut ScrollSpeedMultiplier>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let up = input.any_pressed([KeyCode::ArrowUp, KeyCode::KeyW]);
    let down = input.any_pressed([KeyCode::ArrowDown, KeyCode::KeyS]);

    let direction = up as i8 - down as i8;

    for mut multiplier in entities {
        multiplier.0 = match direction {
            1 => constants::player::ACCELERATION_MULT,
            -1 => constants::player::DECELERATION_MULT,
            _ => 1.0,
        };
    }
}
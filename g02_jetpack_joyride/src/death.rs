use crate::prelude::*;

const DEAD_FALL_SPEED: f32 = 200.0;

#[derive(Component)]
pub struct Dead;

pub fn fall_dead_characters(
    characters: Query<&mut Transform, With<Dead>>,
    time: Res<Time>,
) {
    for mut transform in characters {
        transform.translation.y -= DEAD_FALL_SPEED * time.delta_secs();
    }
}

// TODO: easing
fn ease_in_back(t: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;

    c3 * t * t * t - c1 * t * t
}
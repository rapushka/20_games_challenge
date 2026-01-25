use crate::prelude::{Component, Deref, DerefMut, Query, Res, Time, Transform, Vec2};

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity(pub Vec2);

pub fn update_velocity(
    mut transforms: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();

    for (mut transform, velocity) in &mut transforms {
        transform.translation.x += velocity.x * delta_time;
        transform.translation.y += velocity.y * delta_time;
    }
}
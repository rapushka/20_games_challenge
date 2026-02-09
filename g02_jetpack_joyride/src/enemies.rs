use crate::prelude::*;

pub use spawn_timer::*;
pub use spawn::*;
pub use enemy_type::*;

mod spawn_timer;
mod spawn;
mod enemy_type;

#[derive(Component)]
pub struct Enemy;

pub fn move_enemies(
    enemies: Query<&mut Transform, With<Enemy>>,
    time: Res<Time>,
) {
    for mut transform in enemies {
        transform.translation.x -= time.delta_secs() * constants::ENEMY_MOVEMENT_SPEED;
    }
}

pub fn despawn_enemies_offscreen(
    mut commands: Commands,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (enemy, transform) in enemies {
        if transform.translation.x < constants::ENEMY_DESPAWN_X {
            commands.entity(enemy).despawn();
        }
    }
}
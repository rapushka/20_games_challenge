use crate::prelude::*;

pub use hit_detection::*;
mod hit_detection;

// -> Player
#[derive(Message)]
pub struct Shoot(pub Entity);

#[derive(Component)]
pub struct Bullet;

pub fn fly_bullets(
    bullets: Query<&mut Transform, With<Bullet>>,
    time: Res<Time>,
) {
    for mut transform in bullets {
        let forward = Vec3::X;
        let direction = transform.rotation * forward;

        let scaled_speed = constants::BULLET_FLY_SPEED * time.delta_secs();
        let movement = direction * scaled_speed;

        transform.translation += movement;
    }
}

pub fn send_despawn_on_bullet_too_low(
    mut message_writer: MessageWriter<BulletHit>,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (bullet, transform) in bullets {
        if transform.translation.y <= constants::BULLET_DESPAWN_Y {
            message_writer.write(BulletHit::new_ground(bullet));
        }
    }
}
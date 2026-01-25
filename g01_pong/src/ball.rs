use std::f32::consts::PI;
use crate::prelude::*;
use rand::prelude::*;

const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 400.0;

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

pub fn spawn(
    mut commands: Commands,
) {
    let sizes = vec2(BALL_DIAMETER, BALL_DIAMETER);
    let direction = random_direction();

    commands.spawn((
        Ball,
        Velocity(direction * BALL_SPEED),
        Collider::new(vec2(0.0, 0.0), sizes),
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.9),
            custom_size: Some(sizes),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, z_order::BALL),
    ));
}

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

fn random_direction() -> Vec2 {
    let mut rng = rand::rng();
    let random_number = rng.random_range(0.0..1.0);
    let random_angle: f32 = random_number * PI * 2.0;

    Vec2::new(random_angle.cos(), random_angle.sin())
        .normalize()
}
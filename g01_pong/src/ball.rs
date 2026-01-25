use std::f32::consts::PI;
use crate::prelude::*;
use rand::prelude::*;

const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 400.0;
const BALL_SPAWN_POINT: Vec3 = vec3(0.0, 0.0, z_order::BALL);

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut, Default)]
pub struct Velocity(pub Vec2);

#[derive(Message)]
pub struct ResetBall;

pub fn spawn(
    mut commands: Commands,
    mut message_writer: MessageWriter<ResetBall>,
) {
    let sizes = vec2(BALL_DIAMETER, BALL_DIAMETER);

    commands.spawn((
        Ball,
        Velocity::default(),
        Collider::new(vec2(0.0, 0.0), sizes),
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.9),
            custom_size: Some(sizes),
            ..default()
        },
        Transform::default(),
    ));

    message_writer.write(ResetBall);
}

pub fn reset_ball(
    mut message_reader: MessageReader<ResetBall>,
    mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    for _ in message_reader.read() {
        for (mut transform, mut velocity) in &mut balls {
            transform.translation = BALL_SPAWN_POINT;
            velocity.0 = random_direction() * BALL_SPEED;
        }
    }
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
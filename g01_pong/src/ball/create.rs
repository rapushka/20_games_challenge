use std::f32::consts::PI;
use bevy::window::PrimaryWindow;
use rand::Rng;
use crate::ball::*;

const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 400.0;
const BALL_SPAWN_POINT: Vec3 = vec3(0.0, 0.0, z_order::BALL);

#[derive(Message)]
pub struct ResetBall;

pub fn spawn(
    mut commands: Commands,
    mut message_writer: MessageWriter<ResetBall>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single().unwrap();
    let sizes = vec2(BALL_DIAMETER, BALL_DIAMETER);
    let offscreen = window.resolution.width() * 0.5 + 100.0;

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

fn random_direction() -> Vec2 {
    let mut rng = rand::rng();
    let random_number = rng.random_range(0.0..1.0);
    let random_angle: f32 = random_number * PI * 2.0;

    Vec2::new(random_angle.cos(), random_angle.sin())
        .normalize()
}
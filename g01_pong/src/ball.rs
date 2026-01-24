use bevy::math::bounding::Aabb2d;
use crate::prelude::*;

const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 400.0;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

pub fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sizes = vec2(BALL_DIAMETER, BALL_DIAMETER);

    commands.spawn((
        Ball,
        Velocity(INITIAL_BALL_DIRECTION.normalize() * BALL_SPEED),
        Collider(Aabb2d::new(
            vec2(0.0, 0.0),
            sizes / 2.0,
        )),
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
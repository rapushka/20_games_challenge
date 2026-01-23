use bevy::window::PrimaryWindow;
use crate::prelude::*;

const PADDLE_SIZE: Vec2 = Vec2::new(25.0, 125.0);

#[derive(Component)]
pub struct Paddle;

#[derive(Component, Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
}

pub fn spawn_paddles(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single().unwrap();
    let half_width = window.resolution.width() * 0.5 - 100.0;

    create_paddle(&mut commands, -half_width, Side::Left);
    create_paddle(&mut commands, half_width, Side::Right);
}

fn create_paddle(
    commands: &mut Commands,
    x_position: f32,
    side: Side,
) {
    commands.spawn((
        Paddle,
        side,
        Collider(PADDLE_SIZE),
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.9),
            custom_size: Some(PADDLE_SIZE),
            ..default()
        },
        Transform::from_xyz(x_position, 0.0, z_order::PADDLE),
    ));
}
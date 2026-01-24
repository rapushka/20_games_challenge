use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use crate::ball::{Ball, Velocity};
use crate::prelude::*;

pub use collider::*;
mod collider;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

struct ShouldReflect {
    x: bool,
    y: bool,
}

pub fn check_collisions(
    balls: Query<(&mut Velocity, &Collider), With<Ball>>,
    walls: Query<&Collider, Without<Ball>>,
) {
    for (mut ball_velocity, ball_collider) in balls {
        for wall_collider in &walls {
            let collision = try_collide(ball_collider.aabb(), wall_collider.aabb());

            let Some(collision) = collision else {
                continue;
            };

            // trigger BallCollided?

            let reflect = reflect(collision, ball_velocity.0);

            if reflect.x {
                ball_velocity.x *= -1.0;
            }

            if reflect.y {
                ball_velocity.y *= -1.0;
            }
        }
    }
}

fn try_collide(ball: Aabb2d, collider: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&collider) {
        return None;
    }

    let closest_point = collider.closest_point(ball.center());
    let offset = ball.center() - closest_point;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0.0 { Collision::Left } else { Collision::Right }
    } else if offset.y > 0.0 { Collision::Top } else { Collision::Bottom };

    Some(side)
}

fn reflect(collision: Collision, velocity: Vec2) -> ShouldReflect {
    let mut reflect = ShouldReflect {
        x: false,
        y: false,
    };

    match collision {
        Collision::Left => reflect.x = velocity.x > 0.0,
        Collision::Right => reflect.x = velocity.x < 0.0,
        Collision::Top => reflect.y = velocity.y < 0.0,
        Collision::Bottom => reflect.y = velocity.y > 0.0,
    };

    reflect
}
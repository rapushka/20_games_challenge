use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use crate::ball::{Ball, Velocity};
use crate::prelude::*;

#[derive(Component)]
pub struct Collider(pub Aabb2d);

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

pub fn check_for_collisions(
    mut commands: Commands,
    balls: Query<(&Transform, &mut Velocity, &Collider), With<Ball>>,
    walls: Query<(Entity, &Transform, &Collider), Without<Ball>>,
) {
    for (ball_transform, mut ball_velocity, ball_collider) in balls {
        for (wall_entity, wall_transform, wall_collider) in &walls { // TODO: why `&` is here?
            let collision = try_collide(ball_collider.0, wall_collider.0);
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
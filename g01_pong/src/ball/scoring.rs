use crate::ball::{Ball, ResetBall};
use crate::bounds::Bounds;
use crate::prelude::*;

pub fn check_scored_ball(
    mut reset_ball: MessageWriter<ResetBall>,
    balls: Query<(&Transform, &Bounds), With<Ball>>,
) {
    for (transform, bounds) in balls {
        if bounds.inside_x(transform.translation.x) {
            continue;
        }

        reset_ball.write(ResetBall);
    }
}
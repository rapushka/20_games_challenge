use crate::ball::Ball;
use crate::bounds::Bounds;
use crate::paddle::Side;
use crate::prelude::*;
use crate::scoring::Scored;

pub fn check_scored_ball(
    mut scored_message: MessageWriter<Scored>,
    balls: Query<(&Transform, &Bounds), With<Ball>>,
) {
    for (transform, bounds) in balls {
        if bounds.inside_x(transform.translation.x) {
            continue;
        }

        let side = if transform.translation.x < 0.0 { Side::Right } else { Side::Left };
        scored_message.write(Scored(side));
    }
}
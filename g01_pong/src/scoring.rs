use crate::ball::ResetBall;
use crate::paddle::Side;
use crate::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct CurrentScore {
    left: u32,
    right: u32,
}

#[derive(Message)]
pub struct Scored(pub Side);

pub fn on_scored(
    mut current_score: ResMut<CurrentScore>,
    mut scored_message: MessageReader<Scored>,
    mut reset_ball_message: MessageWriter<ResetBall>,
) {
    for scored in scored_message.read() {
        match scored.0 {
            Side::Left => current_score.left += 1,
            Side::Right => current_score.right += 1,
        }

        // dbg!(&current_score);

        reset_ball_message.write(ResetBall);
    }
}
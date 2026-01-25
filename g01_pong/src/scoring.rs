use crate::ball::ResetBall;
use crate::paddle::Side;
use crate::prelude::*;

#[derive(Message)]
pub struct Scored(pub Side);

pub fn on_scored(
    mut scored_message: MessageReader<Scored>,
    mut reset_ball_message: MessageWriter<ResetBall>,
) {
    for scored in scored_message.read() {
        reset_ball_message.write(ResetBall);
    }
}
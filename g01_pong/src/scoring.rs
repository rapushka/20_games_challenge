use crate::ball::ResetBall;
use crate::paddle::Side;
use crate::prelude::*;

pub use create::*;

mod create;

#[derive(Resource, Default, Debug)]
pub struct CurrentScore {
    left: u32,
    right: u32,
}

#[derive(Component)]
pub struct ScoreView(pub Side);

#[derive(Message, Copy, Clone)]
pub struct Scored(pub Side);

pub fn on_scored(
    mut current_score: ResMut<CurrentScore>,
    mut scored_message: MessageReader<Scored>,
    mut reset_ball_message: MessageWriter<ResetBall>,
    mut views: Query<(&ScoreView, &mut Text2d)>,
) {
    for scored in scored_message.read() {
        let scored_side = scored.0;
        match scored_side {
            Side::Left => current_score.left += 1,
            Side::Right => current_score.right += 1,
        };
        let new_score = match scored_side {
            Side::Left => current_score.left,
            Side::Right => current_score.right,
        };

        for (score_view, mut text) in &mut views {
            if scored_side != score_view.0 {
                continue;
            }

            text.0 = new_score.to_string();
        }

        reset_ball_message.write(ResetBall);
    }
}
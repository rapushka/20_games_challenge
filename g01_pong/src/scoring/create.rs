use crate::paddle::Side;
use crate::prelude::*;
use crate::scoring::ScoreView;

const SCORE_POSITION: Vec3 = vec3(250.0, 150.0, z_order::SCORE);

pub fn spawn_score_views(
    mut commands: Commands,
) {
    spawn(&mut commands, Side::Left);
    spawn(&mut commands, Side::Right);
}

fn spawn(
    commands: &mut Commands,
    side: Side,
) {
    let sign = if side == Side::Right { 1.0 } else { -1.0 };
    let position = SCORE_POSITION.with_x(SCORE_POSITION.x * sign);

    commands.spawn((
        ScoreView(side),
        Text2d::new("0"),
        TextFont {
            font_size: 128.0,
            ..Default::default()
        },
        TextColor(Color::srgb(0.8, 0.8, 0.8)),
        Transform::from_translation(position),
    ));
}
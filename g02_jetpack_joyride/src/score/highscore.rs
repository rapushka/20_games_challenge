use crate::collision_detection::PlayerTouchedEnemy;
use crate::game::IsGameStarted;
use crate::prelude::*;
use crate::score::{Score, ScoreView};
use crate::utils;

#[derive(Component)]
pub struct HighscoreView;

#[derive(Resource, Reflect, Clone, Eq, PartialEq, Debug, Default)]
pub struct Highscore(u32);

pub fn spawn_highscore_view(
    mut commands: Commands,
    highscore: Res<Highscore>,
) {
    let x = constants::CANVAS_HALF_SIZE.x - 50.0;
    let y = constants::MIN_Y - 13.5;

    let text = format!("Highscore: {}", highscore.0);
    commands.spawn((
        utils::new_name("Highscore view"),
        HighscoreView,
        Text2d::new(text),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextLayout::new(Justify::Right, LineBreak::NoWrap),
        Transform {
            translation: vec3(x, y, z_order::UI),
            scale: Vec3::splat(0.15),
            ..default()
        },
    ));
}

pub fn update_highscore_view(
    highscore: Res<Highscore>,
    score_views: Query<&mut Text2d, With<HighscoreView>>,
) {
    for mut text in score_views {
        text.0 = format!("Highscore: {}", highscore.0);
    }
}

pub fn update_highscore(
    mut message: MessageReader<PlayerTouchedEnemy>,
    mut highscore: ResMut<Highscore>,
    score: Res<Score>,
) {
    for _ in message.read() {
        if score.value() > highscore.0 {
            highscore.set_if_neq(Highscore(score.value()));
        }
    }
}
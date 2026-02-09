use crate::game::IsGameStarted;
use crate::prelude::*;
use crate::utils;

pub use core::*;
pub use highscore::*;
mod core;
mod highscore;

#[derive(Component)]
pub struct ScoreView;

pub fn spawn_score_view(
    mut commands: Commands,
) {
    commands.spawn((
        utils::new_name("score view"),
        ScoreView,
        Text2d::new("Press to start!"),
        TextFont {
            font_size: 64.0,
            ..default()
        },
        Transform {
            translation: vec3(0.0, constants::MAX_Y, z_order::UI),
            scale: Vec3::splat(0.2),
            ..default()
        },
    ));
}

pub fn increase_score(
    is_game_started: Res<IsGameStarted>,
    mut score: ResMut<Score>,
    time: Res<Time>,
) {
    if !is_game_started.is_started() {
        return;
    }

    score.tick_time(time.delta());
}

pub fn update_score_view(
    score: Res<Score>,
    score_views: Query<&mut Text2d, With<ScoreView>>,
    is_game_started: Res<IsGameStarted>,
) {
    for mut text in score_views {
        text.0 = if is_game_started.is_started() {
            score.string()
        } else {
            "Press Space to Start".to_string()
        };
    }
}
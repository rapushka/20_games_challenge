use bevy_simple_prefs::Prefs;
use crate::prelude::*;
use crate::score::Highscore;

#[derive(Prefs, Reflect, Default)]
pub struct PlayerPrefs {
    highscore: Highscore,
}
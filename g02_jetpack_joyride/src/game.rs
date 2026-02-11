use crate::death::Dead;
use crate::player::*;
use crate::prelude::*;

#[derive(Resource, Default)]
pub struct IsGameStarted(bool);

impl IsGameStarted {
    pub fn start(&mut self) {
        self.0 = true;
    }

    pub fn stop(&mut self) {
        self.0 = false;
    }

    pub fn is_started(&self) -> bool {
        self.0
    }
}

pub fn start_game_on_first_click(
    mut is_game_started: ResMut<IsGameStarted>,
    alive_players: Query<(), (With<Player>, Without<Dead>)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if alive_players.is_empty() {
        return;
    }

    if keyboard.just_pressed(constants::ASCEND_BUTTON) {
        is_game_started.start();
    }
}
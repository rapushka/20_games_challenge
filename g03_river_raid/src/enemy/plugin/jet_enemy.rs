use crate::constants::level;
use crate::level::RiverBank;
use crate::position::{WorldPosition, ZOrder};
use crate::prelude::*;
use crate::random::Random;

pub struct JetEnemyPlugin;

impl Plugin for JetEnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Initialize), (
                spawn_jet,
            ))
        ;
    }
}

fn spawn_jet(
    mut random: ResMut<Random>,
) {
    let range_between_jets = 5..15;
    let mut counter = random.in_range(&range_between_jets);

    for line_index in 0..100 {
        counter -= 1;

        if counter <= 0 {
            println!("TODO: spawn jet on line {}", line_index);
            counter = random.in_range(&range_between_jets);
        }
    }
}
use crate::player::{Player};
use crate::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Ascending;

pub fn ascend_player(
    players: Query<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let is_pressed = keyboard.pressed(constants::ASCEND_BUTTON);

    if !is_pressed {
        return;
    }

    for mut transform in players {
        let y = transform.translation.y;

        transform.translation.y = (y + constants::ASCENDING_SPEED * time.delta_secs())
            .clamp(constants::MIN_Y, constants::MAX_Y);
    }
}

pub fn descent_player(
    players: Query<&mut Transform, (With<Player>, Without<Ascending>)>,
    time: Res<Time>,
) {
    for mut transform in players {
        let y = transform.translation.y;

        transform.translation.y = (y - constants::DESCENDING_SPEED * time.delta_secs())
            .clamp(constants::MIN_Y, constants::MAX_Y);
    }
}

pub fn update_is_ascending(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let is_pressed = keyboard.pressed(constants::ASCEND_BUTTON);

    for player in players {
        if is_pressed {
            commands.entity(player).insert(Ascending);
        } else {
            commands.entity(player).remove::<Ascending>();
        }
    }
}

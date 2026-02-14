use crate::prelude::*;

pub mod plugin;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn((
        Name::new("Player"),
        Player,
        Sprite {
            color: utils::from_hex("#ffffff"),
            custom_size: Some(vec2(100.0, 150.0)),
            ..default()
        },
        Transform::default(),
    ));
}

pub fn move_player_x(
    players: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time<Fixed>>,
) {
    for mut transform in players {
        let mut direction = 0.0;

        if input.pressed(KeyCode::ArrowLeft) {
            direction = -1.0;
        }

        if input.pressed(KeyCode::ArrowRight) {
            direction = 1.0;
        }

        let scaled_speed = constants::player::HORIZONTAL_SPEED * time.delta_secs();
        transform.translation.x += scaled_speed * direction;
    }
}
use crate::animation::animate_sprites;
use crate::prelude::*;
use crate::prelude::KeyCode;

pub mod z_order {
    pub const BACKGROUND: f32 = 0.0;
    pub const PLAYER: f32 = 10.0;
}

pub mod constants {
    use crate::prelude::*;

    pub const PLAYER_X: f32 = -450.0;

    pub const GROUND_Y: f32 = MIN_Y;
    pub const MIN_Y: f32 = -250.0;

    pub const MAX_Y: f32 = 250.0;
    pub const ASCENDING_SPEED: f32 = 250.0;

    pub const DESCENDING_SPEED: f32 = 250.0;

    pub const BUTTON: KeyCode = KeyCode::Space;
}

pub mod prelude;
mod animation;

#[derive(Component)]
struct Player;

#[derive(Component)]
#[component(storage = "SparseSet")]
struct Ascending;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
        )

        .add_systems(Startup, (
            spawn_camera,
            spawn_player,
        ).chain())

        .add_systems(Update, (
            ascend_player,
            update_is_ascending,
            descent_player,

            // view
            animate_sprites,
        ).chain())

        .run()
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2d);
}

fn spawn_player(
    mut commands: Commands,
) {
    commands.spawn((
        Player,
        Sprite {
            color: color_from("#8888ff"),
            custom_size: Some(vec2(50.0, 125.0)),
            ..default()
        },
        Transform::from_xyz(constants::PLAYER_X, constants::GROUND_Y, z_order::PLAYER),
    ));
}

fn ascend_player(
    players: Query<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let is_pressed = keyboard.pressed(constants::BUTTON);

    if !is_pressed {
        return;
    }

    for mut transform in players {
        let y = transform.translation.y;

        transform.translation.y = (y + constants::ASCENDING_SPEED * time.delta_secs())
            .clamp(constants::MIN_Y, constants::MAX_Y);
    }
}

fn descent_player(
    players: Query<&mut Transform, (With<Player>, Without<Ascending>)>,
    time: Res<Time>,
) {
    for mut transform in players {
        let y = transform.translation.y;

        transform.translation.y = (y - constants::DESCENDING_SPEED * time.delta_secs())
            .clamp(constants::MIN_Y, constants::MAX_Y);
    }
}

fn update_is_ascending(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let is_pressed = keyboard.pressed(constants::BUTTON);

    for player in players {
        if is_pressed {
            commands.entity(player).insert(Ascending);
        } else {
            commands.entity(player).remove::<Ascending>();
        }
    }
}

fn color_from(hex: &'static str) -> Color {
    Srgba::hex(hex).unwrap().into()
}
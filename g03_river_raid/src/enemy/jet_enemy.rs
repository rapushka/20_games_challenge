use crate::collision_detection::{Collider, Obstacle};
use crate::position::{WorldPosition, ZOrder};
use crate::prelude::*;
use crate::random::Random;

mod jet_direction;
use crate::active_flag::Active;
use crate::enemy::Enemy;
pub use jet_direction::*;

#[derive(Component)]
pub struct Jet {
    direction: JetDirection,
}

pub fn spawn_jets_on_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut random: ResMut<Random>,
) {
    let range_between_jets = 2..10;
    let mut counter = random.in_range(&range_between_jets);

    for line_index in 0..100 {
        counter -= 1;

        let y = utils::index_to_position(line_index);

        if counter <= 0 {
            let direction = JetDirection::new_random(&mut random);

            spawn_jet(y, direction, &mut commands, &asset_server);
            counter = random.in_range(&range_between_jets);
        }
    }
}

pub fn move_jets(
    jets: Query<(&Jet, &mut WorldPosition), With<Active>>,
    time: Res<Time<Virtual>>,
) {
    let speed = constants::enemies::JET_MOVEMENT_SPEED;
    let delta_time = time.delta_secs();

    for (Jet { direction }, mut position) in jets {
        let direction = direction.as_f32();

        position.x += direction * speed * delta_time;
    }
}

fn spawn_jet(
    y: f32,
    direction: JetDirection,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let image = asset_server.load(asset_path::ENEMY_JET);
    let spawn_offset = constants::level::LEVEL_HALF_WIDTH + constants::enemies::JET_SPAWN_OFFSET;
    let x = -direction.as_f32() * spawn_offset;

    let mut sprite = Sprite::from_image(image);
    sprite.flip_x = direction == JetDirection::Right;

    commands.spawn((
        Name::new("Enemy_Jet"),
        Enemy,
        Jet { direction },
        Obstacle,
        sprite,
        WorldPosition::new(x, y),
        ZOrder::Enemies,
        Collider::new(vec2(110.0, 40.0), vec2(0.0, 0.0)),
    ));
}
use crate::collision_detection::{Collider, Obstacle};
use crate::position::{WorldPosition, ZOrder};
use crate::prelude::*;
use crate::random::Random;

mod jet_direction;
use crate::active_flag::Active;
use crate::enemy::{Enemy, MovementSpeed};
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
            let direction = JetDirection::new_random_direction(&mut random);
            let speed = random.in_range_f32(&constants::enemies::JET_MOVEMENT_SPEED_RANGE);

            spawn_jet(y, direction, speed, &mut commands, &asset_server);
            counter = random.in_range(&range_between_jets);
        }
    }
}

pub fn move_jets(
    jets: Query<(&Jet, &mut WorldPosition, &MovementSpeed), With<Active>>,
    time: Res<Time<Virtual>>,
) {
    let delta_time = time.delta_secs();

    for (Jet { direction }, mut position, MovementSpeed(speed)) in jets {
        let direction = direction.as_f32();

        position.x += direction * speed * delta_time;
    }
}

pub fn despawn_offscreen_jets(
    mut commands: Commands,
    jets: Query<(Entity, &Jet, &mut WorldPosition)>,
) {
    for (entity, Jet { direction }, position) in jets {
        let offscreen_x = constants::enemies::JET_OFFSCREEN_POSITION;

        let should_despawn = match direction {
            JetDirection::Left => position.x <= offscreen_x,
            JetDirection::Right => position.x >= offscreen_x,
        };

        if should_despawn {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_jet(
    y: f32,
    direction: JetDirection,
    speed: f32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let image = asset_server.load(asset_path::ENEMY_JET);
    let spawn_offset = constants::enemies::JET_OFFSCREEN_POSITION;

    // Direction is negated since we need the direction FROM which jet will move
    let x = -direction.as_f32() * spawn_offset;

    let mut sprite = Sprite::from_image(image);
    sprite.flip_x = direction == JetDirection::Right;

    commands.spawn((
        Name::new("Enemy_Jet"),
        Enemy,
        MovementSpeed(speed),
        Jet { direction },
        Obstacle,
        sprite,
        WorldPosition::new(x, y),
        ZOrder::Enemies,
        Collider::new(vec2(110.0, 40.0), vec2(0.0, 0.0)),
    ));
}
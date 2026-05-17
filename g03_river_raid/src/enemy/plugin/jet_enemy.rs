use crate::collision_detection::{Collider, Obstacle};
use crate::enemy::plugin::Enemy;
use crate::position::{WorldPosition, ZOrder};
use crate::prelude::*;
use crate::random::Random;

mod jet_direction;
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

fn spawn_jet(
    y: f32,
    direction: JetDirection,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let image = asset_server.load(asset_path::ENEMY_JET);
    let x = 0.0; // TODO: put jets offscreen

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
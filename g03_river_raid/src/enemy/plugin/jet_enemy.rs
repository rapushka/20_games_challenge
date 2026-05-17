use crate::collision_detection::Collider;
use crate::enemy::plugin::Enemy;
use crate::position::{WorldPosition, ZOrder};
use crate::prelude::*;
use crate::random::Random;

#[derive(Component)]
pub struct Jet;

pub fn spawn_jets_on_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut random: ResMut<Random>,
) {
    let range_between_jets = 5..15;
    let mut counter = random.in_range(&range_between_jets);

    for line_index in 0..100 {
        counter -= 1;

        let y = utils::index_to_position(line_index);

        if counter <= 0 {
            spawn_jet(y, &mut commands, &asset_server);

            counter = random.in_range(&range_between_jets);
        }
    }
}

fn spawn_jet(
    y: f32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let image = asset_server.load(asset_path::ENEMY_JET);
    let x = 0.0; // TODO: put jets offscreen

    commands.spawn((
        Name::new("Enemy_Jet"),
        Enemy,
        Jet,
        Sprite::from_image(image),
        WorldPosition::new(x, y),
        ZOrder::Enemies,
        Collider::new(vec2(25.0, 85.0), vec2(0.0, -10.0)), // TODO: update collider
    ));
}
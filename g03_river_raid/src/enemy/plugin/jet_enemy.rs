use crate::collision_detection::Collider;
use crate::enemy::plugin::Enemy;
use crate::position::{WorldPosition, ZOrder};
use crate::prelude::*;
use crate::random::Random;

#[derive(Component)]
pub struct Jet;

pub struct JetEnemyPlugin;

impl Plugin for JetEnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Initialize), (
                spawn_jets_on_level,
            ))
        ;
    }
}

fn spawn_jets_on_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut random: ResMut<Random>,
) {
    let range_between_jets = 5..15;
    let mut counter = random.in_range(&range_between_jets);

    for line_index in 0..100 {
        counter -= 1;

        if counter <= 0 {
            spawn_jet(&mut commands, &asset_server);

            counter = random.in_range(&range_between_jets);
        }
    }
}

fn spawn_jet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let image = asset_server.load(asset_path::ENEMY_JET);

    commands.spawn((
        Name::new("Enemy_Jet"),
        Enemy,
        Jet,
        Sprite::from_image(image),
        WorldPosition::ZERO, // TODO
        ZOrder::Enemies,
        Collider::new(vec2(25.0, 85.0), vec2(0.0, -10.0)), // TODO
    ));
}
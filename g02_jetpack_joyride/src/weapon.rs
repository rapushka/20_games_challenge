use std::time::Duration;
use rand::Rng;
use crate::prelude::*;

pub use spawn::*;
use crate::bullet::{Bullet, Shoot};
use crate::collision_detection::Collider;
use crate::player::{Ascending, Player};
use crate::utils;
use crate::utils::from_hex;

mod spawn;

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct Muzzle;

// Expected to be on Player entity
#[derive(Component)]
pub struct Inventory {
    weapon: Entity,
    muzzle: Entity,
}

#[derive(Component, DerefMut, Deref)]
pub struct ShootTimer(pub Timer);

impl ShootTimer {
    pub fn new() -> Self {
        let duration = Duration::from_secs_f32(constants::SHOOT_BULLET_DELAY_S);
        Self(Timer::new(duration, TimerMode::Repeating))
    }
}

pub fn tick_shooting_timer_while_ascending(
    mut shoot_message: MessageWriter<Shoot>,
    players: Query<(Entity, &mut ShootTimer), (With<Player>, With<Ascending>)>,
    time: Res<Time>,
) {
    for (player, mut timer) in players {
        timer.tick(time.delta());

        if timer.just_finished() {
            shoot_message.write(Shoot(player));
        }
    }
}

pub fn shoot_bullets(
    mut shoot_message: MessageReader<Shoot>,
    mut commands: Commands,
    inventories: Query<&Inventory>,
    transforms: Query<&GlobalTransform>,
) {
    for player in shoot_message.read() {
        let inventory = inventories.get(player.0).unwrap();
        let weapon_transform = transforms.get(inventory.weapon).unwrap();
        let muzzle_transform = transforms.get(inventory.muzzle).unwrap();

        let mut rotation = weapon_transform.rotation();
        let max_rad = constants::BULLET_MAX_SPREAD.to_radians();
        rotation.z += rand::rng().random_range(-max_rad..=max_rad);

        commands.spawn((
            utils::new_name("Bullet"),
            Bullet,
            Sprite {
                custom_size: Some(constants::BULLET_SIZE),
                color: from_hex(constants::BULLET_COLOR),
                ..default()
            },
            Transform {
                translation: muzzle_transform.translation(),
                rotation,
                ..default()
            },
            Collider::new(2.5, Vec2::ZERO),
        ));
    }
}
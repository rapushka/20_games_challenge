use std::time::Duration;
use crate::prelude::*;

pub use spawn::*;
use crate::bullet::{Bullet, Shoot};
use crate::player::{Ascending, Player};
use crate::utils;
use crate::utils::from_hex;

mod spawn;

#[derive(Component)]
pub struct Weapon;

#[derive(Component)]
pub struct Muzzle;

// Player -> Muzzle
#[derive(Component)]
pub struct HeldMuzzle(pub Entity);

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

pub fn shoot_bullet(
    mut shoot_message: MessageReader<Shoot>,
    mut commands: Commands,
    held_muzzle: Query<&HeldMuzzle>,
    transforms: Query<&GlobalTransform>,
) {
    for player in shoot_message.read() {
        let muzzle_id = held_muzzle.get(player.0).unwrap().0;
        let muzzle_transform = transforms.get(muzzle_id).unwrap();

        commands.spawn((
            utils::new_name("Bullet"),
            Bullet,
            Sprite {
                custom_size: Some(vec2(3.0, 7.5)),
                color: from_hex("#ffffff"),
                ..default()
            },
            Transform {
                translation: muzzle_transform.translation(),
                ..default()
            },
        ));
    }
}
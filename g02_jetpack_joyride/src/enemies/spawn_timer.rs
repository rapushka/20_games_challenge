use crate::prelude::*;
use crate::utils;
use rand::Rng;
use crate::enemies::EnemyType;
use crate::game::IsGameStarted;

const SPAWN_INTERVALS: f32 = 0.5;

#[derive(Message)]
pub struct SpawnEnemy(pub EnemyType);

#[derive(Resource, Deref, DerefMut)]
pub struct EnemySpawnerTimer(Timer);

impl Default for EnemySpawnerTimer {
    fn default() -> Self {
        Self(utils::new_timer(SPAWN_INTERVALS))
    }
}

pub fn tick_enemy_spawn_timer(
    mut message_writer: MessageWriter<SpawnEnemy>,
    mut spawner_timer: ResMut<EnemySpawnerTimer>,
    is_game_started: Res<IsGameStarted>,
    time: Res<Time>,
) {
    if !is_game_started.is_started() {
        return;
    }

    spawner_timer.tick(time.delta());

    if spawner_timer.just_finished() {
        let random = rand::rng().random_range(0.0..=1.0f32);

        let enemy_type = if random <= 0.75 {
            EnemyType::Flying
        } else if random <= 0.85 {
            EnemyType::OnGround1
        } else {
            EnemyType::OnGround2
        };

        message_writer.write(SpawnEnemy(enemy_type));
    }
}
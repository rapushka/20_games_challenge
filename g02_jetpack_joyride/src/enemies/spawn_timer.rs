use std::time::Duration;
use crate::prelude::*;
use crate::utils;
use rand::Rng;
use crate::enemies::EnemyType;
use crate::game::IsGameStarted;

#[derive(Message)]
pub struct SpawnEnemy(pub EnemyType);

#[derive(Resource)]
pub struct EnemySpawnTimer {
    timer: Timer,
    init_duration: Duration,
}

impl EnemySpawnTimer {
    pub fn new(duration_seconds: f32) -> Self {
        Self {
            init_duration: Duration::from_secs_f32(duration_seconds),
            timer: utils::new_repeat_timer(duration_seconds),
        }
    }

    pub fn reset(&mut self) {
        let deviation = rand::rng().random_range(constants::ENEMY_SPAWN_DEVIATION);
        let deviation = Duration::from_secs_f32(deviation);

        self.timer.set_duration(self.init_duration + deviation);
    }
}

pub fn tick_enemy_spawn_timer(
    mut message_writer: MessageWriter<SpawnEnemy>,
    mut spawner_timer: ResMut<EnemySpawnTimer>,
    is_game_started: Res<IsGameStarted>,
    time: Res<Time>,
) {
    if !is_game_started.is_started() {
        return;
    }

    spawner_timer.timer.tick(time.delta());

    if spawner_timer.timer.just_finished() {
        spawner_timer.reset();
        let random = rand::rng().random_range(0.0..=1.0f32);

        let enemy_type = if random <= 0.60 {
            EnemyType::Fly
        } else if random <= 0.80 {
            EnemyType::Worm
        } else {
            EnemyType::YellowGuy
        };

        message_writer.write(SpawnEnemy(enemy_type));
    }
}
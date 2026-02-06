use std::time::Duration;
use crate::prelude::{Resource, Timer, TimerMode};

const SCORE_INCREMENT_INTERVAL: f32 = 0.1;
const SCORE_INCREMENT_VALUE: u32 = 1;

#[derive(Resource)]
pub struct Score {
    value: u32,
    timer: Timer,
}

impl Default for Score {
    fn default() -> Self {
        let tick_duration = Duration::from_secs_f32(SCORE_INCREMENT_INTERVAL);
        Self {
            value: 0,
            timer: Timer::new(tick_duration, TimerMode::Repeating),
        }
    }
}

impl Score {
    pub fn value(&self) -> String {
        self.value.to_string()
    }

    pub fn tick_time(&mut self, delta: Duration) {
        self.timer.tick(delta);

        if self.timer.just_finished() {
            self.value += SCORE_INCREMENT_VALUE;
        }
    }
}
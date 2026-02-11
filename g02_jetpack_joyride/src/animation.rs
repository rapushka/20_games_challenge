use std::time::Duration;
use crate::prelude::*;
use crate::utils;

#[derive(Component)]
pub struct Animator {
    first_index: usize,
    last_index: usize,

    timer: Timer,
}

impl Animator {
    pub fn new(fps: f32, first_index: usize, last_index: usize) -> Self {
        Self {
            timer: utils::new_repeat_timer(1.0 / fps),
            first_index,
            last_index,
        }
    }

    pub fn one_frame(index: usize) -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(0), TimerMode::Repeating),
            first_index: index,
            last_index: index,
        }
    }

    pub fn first_index(&self) -> usize {
        self.first_index
    }

    pub fn next_index(&self, current_index: usize) -> usize {
        if current_index == self.last_index {
            self.first_index
        } else {
            current_index + 1
        }
    }
}

pub fn animate_sprites(
    animators: Query<(&mut Animator, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut animator, mut sprite) in animators {
        animator.timer.tick(time.delta());

        if animator.timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = animator.next_index(atlas.index);
        }
    }
}
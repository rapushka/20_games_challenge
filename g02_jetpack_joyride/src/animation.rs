use crate::prelude::*;

#[derive(Component)]
pub struct Animator {
    first_index: usize,
    last_index: usize,

    timer: Timer,
}

impl Animator {
    pub fn new(fps: f32, first_index: usize, last_index: usize) -> Self {
        Animator {
            timer: Timer::from_seconds(1.0 / fps, TimerMode::Repeating),
            first_index,
            last_index,
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
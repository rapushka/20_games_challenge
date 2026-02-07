use crate::animation::Animator;
use crate::prelude::*;

pub enum EnemyType {
    Flying,
    OnGround1,
    OnGround2,
}

impl EnemyType {
    pub fn new_animator(&self) -> Animator {
        // Animator::new(7.0, 0, 1)
        match self {
            EnemyType::Flying => Animator::new(7.0, 4, 5),
            EnemyType::OnGround1 => Animator::new(7.0, 0, 1),
            EnemyType::OnGround2 => Animator::new(7.0, 8, 9),
        }
    }

    pub fn spawn_y(&self) -> f32 {
        match self {
            EnemyType::Flying => constants::GROUND_Y + 50.0, // TODO: randomize
            EnemyType::OnGround1 => constants::GROUND_Y,
            EnemyType::OnGround2 => constants::GROUND_Y,
        }
    }
}
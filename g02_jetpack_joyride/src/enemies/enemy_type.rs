use crate::animation::Animator;
use crate::collision_detection::Collider;
use crate::prelude::*;

pub enum EnemyType {
    Fly,
    Worm,
    YellowGuy,
}

impl EnemyType {
    pub fn spawn_y(&self) -> f32 {
        match self {
            EnemyType::Fly => constants::GROUND_Y + 50.0, // TODO: randomize
            EnemyType::Worm => constants::GROUND_Y,
            EnemyType::YellowGuy => constants::GROUND_Y + 1.0,
        }
    }

    pub fn new_animator(&self) -> Animator {
        // Animator::new(7.0, 0, 1)
        match self {
            EnemyType::Fly => Animator::new(7.0, 4, 5),
            EnemyType::Worm => Animator::new(7.0, 0, 1),
            EnemyType::YellowGuy => Animator::new(7.0, 8, 9),
        }
    }

    pub fn new_collider(&self) -> Collider {
        match self {
            EnemyType::Fly => Collider::new(5.0, vec2(0.0, 0.0)),
            EnemyType::Worm => Collider::new(5.0, vec2(0.0, -8.0)),
            EnemyType::YellowGuy => Collider::new(5.0, vec2(0.0, -6.0)),
        }
    }
}
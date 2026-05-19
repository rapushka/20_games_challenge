use crate::random::Random;

#[derive(Eq, PartialEq)]
pub enum JetDirection {
    Left,
    Right,
}

impl JetDirection {
    pub fn new_random_direction(random: &mut Random) -> Self {
        if random.flip_a_coin() {
            JetDirection::Left
        } else {
            JetDirection::Right
        }
    }

    pub fn as_f32(&self) -> f32 {
        match self {
            JetDirection::Left => -1.0,
            JetDirection::Right => 1.0,
        }
    }
}
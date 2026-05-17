use crate::random::Random;

#[derive(Eq, PartialEq)]
pub enum JetDirection {
    Left,
    Right,
}

impl JetDirection {
    pub fn new_random(random: &mut Random) -> Self {
        if random.flip_a_coin() {
            JetDirection::Left
        } else {
            JetDirection::Right
        }
    }
}
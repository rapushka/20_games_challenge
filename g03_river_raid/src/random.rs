use std::ops::Range;
use rand::prelude::*;
use crate::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct Random(StdRng);

impl Default for Random {
    fn default() -> Self {
        let rng = StdRng::seed_from_u64(375);
        Random(rng)
    }
}

impl Random {
    pub fn pick<T: Clone>(&mut self, variants: Vec<T>) -> T {
        let random_index = self.random_range(0..variants.len());
        variants[random_index].clone()
    }

    pub fn in_range(&mut self, range: &Range<i32>) -> i32 {
        self.0.random_range(range.clone())
    }

    pub fn in_range_f32(&mut self, range: &Range<f32>) -> f32 {
        self.0.random_range(range.clone())
    }

    pub fn flip_a_coin(&mut self) -> bool {
        self.0.random_bool(0.5)
    }
}

pub fn init_random(
    mut commands: Commands,
) {
    commands.insert_resource(Random::default());
}
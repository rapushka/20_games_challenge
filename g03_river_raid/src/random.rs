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
}
use crate::prelude::*;

#[derive(Message)]
pub struct Collision {
    // Who collided
    subject: Entity,
    // With What collided
    object: Entity,
}

impl Collision {
    pub fn new(subject: Entity, object: Entity) -> Self {
        Self { subject, object }
    }

    pub fn subject(&self) -> Entity {
        self.subject
    }

    pub fn object(&self) -> Entity {
        self.object
    }
}
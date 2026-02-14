use crate::prelude::*;

// -> Bullet
#[derive(Message)]
pub struct BulletHit {
    pub bullet: Entity,
}

impl BulletHit {
    pub fn new(bullet: Entity) -> Self {
        Self { bullet }
    }
}

pub fn despawn_hit_bullets(
    mut commands: Commands,
    mut message_reader: MessageReader<BulletHit>,
) {
    for BulletHit { bullet } in message_reader.read() {
        commands.entity(*bullet).try_despawn();
    }
}
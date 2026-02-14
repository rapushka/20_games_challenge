use crate::prelude::*;

#[derive(Eq, PartialEq)]
pub enum HitType {
    Ground,
    Enemy,
}

// -> Bullet
#[derive(Message)]
pub struct BulletHit {
    bullet: Entity,
    hit_type: HitType,
}

impl BulletHit {
    pub fn new_ground(bullet: Entity) -> Self {
        Self {
            bullet,
            hit_type: HitType::Ground,
        }
    }

    pub fn new_enemy(bullet: Entity) -> Self {
        Self {
            bullet,
            hit_type: HitType::Enemy,
        }
    }
}

pub fn despawn_hit_bullets(
    mut commands: Commands,
    mut message_reader: MessageReader<BulletHit>,
) {
    for message in message_reader.read() {
        commands.entity(message.bullet).try_despawn();
    }
}

pub fn play_sound_on_bullet_hit_enemy(
    mut commands: Commands,
    mut message_reader: MessageReader<BulletHit>,
    asset_server: Res<AssetServer>,
) {
    let sound = asset_server.load(asset_path::ENEMY_HIT_SOUND);

    for message in message_reader.read() {
        if message.hit_type != HitType::Enemy {
            continue;
        }

        commands.spawn((
            AudioPlayer::new(sound.clone()),
            PlaybackSettings::DESPAWN,
        ));
    }
}
use crate::collision_detection::Collider;
use crate::enemies::{Enemy, SpawnEnemy};
use crate::prelude::*;
use crate::utils;

pub fn spawn_enemy(
    mut message_reader: MessageReader<SpawnEnemy>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for SpawnEnemy(enemy_type) in message_reader.read() {
        let image = asset_server.load(asset_path::ENEMIES_IMAGE);
        let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 4, 4, Some(UVec2::splat(1)), None);
        let layout_handle = texture_atlas_layouts.add(layout);

        let spawn_y = enemy_type.spawn_y();
        let animator = enemy_type.new_animator();

        let mut sprite = Sprite::from_atlas_image(
            image,
            TextureAtlas {
                layout: layout_handle,
                index: animator.first_index(),
            },
        );
        sprite.flip_x = true;
        let spawn_position = vec3(constants::ENEMY_SPAWN_X, spawn_y, z_order::ENEMIES);

        commands.spawn((
            utils::new_name("Enemy"),
            Enemy,
            ( // view
              sprite,
              animator,
              Transform {
                  translation: spawn_position,
                  rotation: Quat::default(),
                  scale: Vec3::splat(1.0),
              },
            ),
            Collider::new(spawn_position.truncate(), 5.0), // TODO: RADIUS
        ));
    }
}
use crate::animation::Animator;
use crate::player::Player;
use crate::prelude::*;
use crate::utils;
pub const FUCHSIA: Srgba = Srgba::rgb(1.0, 0.0, 1.0);

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load(asset_path::PLAYER_IMAGE);
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(25), 4, 1, None, None);
    let layout_handle = texture_atlas_layouts.add(layout);

    let animator = Animator::new(7.0, 0, 1);

    commands.spawn((
        utils::new_name("Player"),
        Player,
        ( // view
          Sprite::from_atlas_image(
              image,
              TextureAtlas {
                  layout: layout_handle,
                  index: animator.first_index(),
              },
          ),
          animator,
          Transform {
              translation: vec3(constants::PLAYER_X, constants::GROUND_Y, z_order::PLAYER),
              rotation: Quat::default(),
              scale: Vec3::splat(1.0),
          },
        ),
    ));
}
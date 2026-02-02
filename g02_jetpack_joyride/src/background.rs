use crate::prelude::*;
use crate::utils;

const BG_SIZES: Vec2 = vec2(240.0, 135.0);

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct ScrollTextureX {
    speed: f32,
    current: f32,
}

impl ScrollTextureX {
    pub fn new(speed: f32) -> Self {
        Self { speed, current: 0.0 }
    }
}

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let image = asset_server.load_with_settings(asset_path::BG_IMAGE, utils::sampler_repeat);

    let mut sprite = Sprite::from_image(image);
    sprite.image_mode = SpriteImageMode::Tiled {
        tile_x: true,
        tile_y: true,
        stretch_value: BG_SIZES.y,
    };

    commands.spawn((
        utils::new_name("background"),
        Background,
        ScrollTextureX::new(constants::LEVEL_SCROLL_SPEED),
        sprite,
        Transform::from_xyz(0.0, 0.5, z_order::BACKGROUND),
    ));
}

pub fn scroll_background(
    backgrounds: Query<(&mut Sprite, &mut ScrollTextureX), With<Background>>,
    time: Res<Time>,
) {
    for (mut sprite, mut scroll) in backgrounds {
        scroll.current += scroll.speed * time.delta_secs();

        let mut rect = Rect::from_corners(Vec2::ZERO, BG_SIZES);
        rect.min.x += scroll.current;
        rect.max.x += scroll.current;

        sprite.rect = Some(rect);

        // sprite.rect = Some(Rect {
        //     min: vec2(scroll.current, 0.0),
        //     max: vec2(scroll.current * BG_SIZES.x, BG_SIZES.y),
        // });
    }
}
use crate::prelude::*;

#[derive(Component)]
pub struct Background;

pub fn spawn_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let image = asset_server.load("environment/bg.png");

    commands.spawn((
        Background,
        Sprite::from_image(image),
    ));
}
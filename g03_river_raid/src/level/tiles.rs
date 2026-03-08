use crate::prelude::*;

use crate::level::TILE_SIZE;

mod getters;

#[derive(Resource)]
pub struct EnvironmentTiles {
    layout: Handle<TextureAtlasLayout>,

    inner_top_left: usize,
    inner_top_center: usize,
    inner_top_right: usize,
    inner_center_left: usize,
    inner_center_center: usize,
    inner_center_right: usize,
    inner_bottom_left: usize,
    inner_bottom_center: usize,
    inner_bottom_right: usize,

    outer_top_left: usize,
    outer_top_center: usize,
    outer_top_right: usize,
    outer_center_left: usize,
    outer_center_center: usize,
    outer_center_right: usize,
    outer_bottom_left: usize,
    outer_bottom_center: usize,
    outer_bottom_right: usize,
}

impl FromWorld for EnvironmentTiles {
    fn from_world(world: &mut World) -> Self {
        let tile_side = TILE_SIZE as u32;
        let size = uvec2(tile_side, tile_side);
        let layout = TextureAtlasLayout::from_grid(size, 7, 3, None, None);

        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        let handle = texture_atlases.add(layout);

        Self {
            layout: handle,

            inner_top_left: 0,
            inner_top_center: 1,
            inner_top_right: 2,
            inner_center_left: 7,
            inner_center_center: 8,
            inner_center_right: 9,
            inner_bottom_left: 14,
            inner_bottom_center: 15,
            inner_bottom_right: 16,
            outer_top_left: 4,
            outer_top_center: 5,
            outer_top_right: 6,
            outer_center_left: 11,
            outer_center_center: 12,
            outer_center_right: 13,
            outer_bottom_left: 18,
            outer_bottom_center: 19,
            outer_bottom_right: 20,
        }
    }
}
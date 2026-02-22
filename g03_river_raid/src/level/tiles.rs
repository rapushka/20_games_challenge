use crate::prelude::*;

pub use getters::*;
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
        let size = uvec2(128, 128);
        let layout = TextureAtlasLayout::from_grid(size, 6, 3, None, None);

        let mut texture_atlases = world
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .unwrap();
        let handle = texture_atlases.add(layout);

        Self {
            layout: handle,

            inner_top_left: 0,
            inner_top_center: 1,
            inner_top_right: 2,
            inner_center_left: 6,
            inner_center_center: 7,
            inner_center_right: 8,
            inner_bottom_left: 12,
            inner_bottom_center: 13,
            inner_bottom_right: 14,
            outer_top_left: 3,
            outer_top_center: 4,
            outer_top_right: 5,
            outer_center_left: 9,
            outer_center_center: 10,
            outer_center_right: 11,
            outer_bottom_left: 15,
            outer_bottom_center: 16,
            outer_bottom_right: 17,
        }
    }
}
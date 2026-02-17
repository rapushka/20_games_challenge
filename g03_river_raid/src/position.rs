use crate::prelude::*;

#[derive(Component, Deref, DerefMut, Default)]
pub struct WorldPosition(Vec2);

impl WorldPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self(vec2(x, y))
    }
}

#[derive(Component, Copy, Clone)]
#[repr(i32)]
pub enum ZOrder {
    Background = -1,
    Player = 10,
}

impl ZOrder {
    pub fn as_f32(&self) -> f32 {
        *self as i32 as f32
    }
}

pub fn update_positions(
    entities: Query<(&mut Transform, &WorldPosition, Option<&ZOrder>)>,
) {
    for (mut transform, position, z_order) in entities {
        transform.translation.x = position.x;
        transform.translation.y = position.y;

        if let Some(z_order) = z_order {
            transform.translation.z = z_order.as_f32();
        }
    }
}
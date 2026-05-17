use bevy::ecs::query::QuerySingleError;
use crate::constants::level;
use crate::prelude::*;

const ERROR_PINK: Color = Color::srgb_u8(255, 0, 255);

pub fn from_hex(hex: &'static str) -> Color {
    if let Ok(color) = Srgba::hex(hex) {
        return color.into();
    }

    error!("Can't parse Color from Hex: {}", hex);
    ERROR_PINK
}

pub trait ResultExt<T> {
    fn zero_or_one(self) -> Option<T>;
}

impl<T> ResultExt<T> for Result<T, QuerySingleError> {
    fn zero_or_one(self) -> Option<T> {
        match self {
            Ok(target) => Some(target),
            Err(QuerySingleError::NoEntities(_)) => None,
            Err(QuerySingleError::MultipleEntities(_)) => panic!("Multiple Entities!"),
        }
    }
}

pub fn index_to_position(index: i32) -> f32 {
    index as f32 * level::TILE_SIZE
}
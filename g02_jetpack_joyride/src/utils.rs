use crate::prelude::*;

const ERROR_PINK: Color = Color::srgb_u8(255, 0, 255);

pub fn from_hex(hex: &'static str) -> Color {
    if let Ok(color) = Srgba::hex(hex) {
        return color.into();
    }

    error!("Can't parse Color from Hex: {}", hex);
    ERROR_PINK
}
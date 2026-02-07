use std::time::Duration;
use bevy::image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor};
use crate::prelude::*;

const ERROR_PINK: Color = Color::srgb_u8(255, 0, 255);

pub fn from_hex(hex: &'static str) -> Color {
    if let Ok(color) = Srgba::hex(hex) {
        return color.into();
    }

    error!("Can't parse Color from Hex: {}", hex);
    ERROR_PINK
}

pub fn sampler_repeat(s: &mut ImageLoaderSettings) {
    *s = ImageLoaderSettings {
        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
            address_mode_u: ImageAddressMode::Repeat,
            address_mode_v: ImageAddressMode::Repeat,
            ..default()
        }),
        ..default()
    };
}

pub fn new_name(name: &'static str) -> Name {
    Name::new(name)
}

pub fn rotation_2d(degrees: f32) -> Quat {
    Quat::from_rotation_z(degrees.to_radians())
}

pub fn new_timer(duration_seconds: f32) -> Timer {
    Timer::new(Duration::from_secs_f32(duration_seconds), TimerMode::Repeating)
}
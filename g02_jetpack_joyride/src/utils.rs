use crate::prelude::*;

pub fn color_from(hex: &'static str) -> Color {
    Srgba::hex(hex).unwrap().into()
}
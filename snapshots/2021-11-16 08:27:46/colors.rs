use crate::extensions::srgb8::*;
use nannou::prelude::*;

pub fn soft_white() -> Hsl {
    // srgb8(249, 248, 245).into_hsl()
    hsl(0.12466522, 0.34505126, 0.9302026)
}

pub fn soft_black() -> Hsl {
    srgb8(26, 26, 22).into_hsl()
}

pub fn cooper_yellow() -> Hsl {
    srgb8(229, 184, 0).into_hsl()
}

pub fn cooper_black() -> Hsl {
    srgb8(63, 47, 11).into_hsl()
}

pub fn cooper_gray() -> Hsl {
    srgb8(222, 225, 213).into_hsl()
}

use crate::extensions::srgb8::*;
use nannou::prelude::*;

pub fn soft_white() -> Hsl {
    // srgb8(249, 248, 245).into_hsl()
    hsl(0.12466522, 0.34505126, 0.9302026)
}

pub fn soft_black() -> Hsl {
    srgb8(26, 26, 22).into_hsl()
}

pub fn whitney_black() -> Hsl {
    srgb8(9, 8, 9).into_hsl()
}

pub fn whitney_white() -> Hsl {
    srgb8(163, 163, 166).into_hsl()
}

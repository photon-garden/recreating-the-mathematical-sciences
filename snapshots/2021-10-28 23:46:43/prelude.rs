use std::ops::RangeInclusive;

pub use crate::colors::*;
pub use crate::components::*;
pub use crate::extensions::*;
pub use crate::grid::*;
pub use crate::snapshot::rand::Rand;
pub use crate::svg::*;

pub use nannou::prelude::*;

pub const ZERO_TO_ONE: RangeInclusive<f32> = 0.0..=1.0;
pub const PI_HALVES: f32 = PI / 2.0;

pub type NumberOfTurns = f32;
pub type NormalizedF32 = f32;

pub struct Model {
    pub snapshot: crate::snapshot::Snapshot,
    pub root_component: crate::components::artwork::Artwork,
    pub container: Rect<f32>,
}

pub struct RenderParams<'a> {
    pub app: &'a App,
    pub rand: &'a mut Rand,
    pub draw: &'a Draw,
    pub model: &'a Model,
    pub container: &'a Rect<f32>,
}

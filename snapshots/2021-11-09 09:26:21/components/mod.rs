use crate::prelude::*;

mod amplitude_modulation_test;
mod lissajous_lines;

pub struct Artwork {}

pub fn new(params: Params) -> Artwork {
    Artwork {}
}

impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        params.draw.background().color(whitney_white());
        lissajous_lines::render(params);
    }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}

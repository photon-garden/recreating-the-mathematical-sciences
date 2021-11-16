use crate::prelude::*;

mod corner_grid;

pub struct Artwork {}

pub fn new(params: Params) -> Artwork {
    Artwork {}
}

impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        params.draw.background().color(soft_white());
        corner_grid::render(params);
    }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}

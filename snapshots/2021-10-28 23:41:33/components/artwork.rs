use crate::prelude::*;

pub struct Artwork {
    rand: Rand,
}

pub fn new(params: Params) -> Artwork {
    Artwork { rand: params.rand }
}

impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        circle::render(params)
    }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}

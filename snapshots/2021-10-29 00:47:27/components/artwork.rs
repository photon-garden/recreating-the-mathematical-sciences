use crate::prelude::*;
use vertical_boundary::*;

pub struct Artwork {
    rand: Rand,
    vertical_boundary: VerticalBoundary,
}

pub fn new(params: Params) -> Artwork {
    Artwork {
        rand: params.rand,
        vertical_boundary: vertical_boundary::new(params.app),
    }
}

impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        circle::render(params);
        self.vertical_boundary.render(params);
    }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}

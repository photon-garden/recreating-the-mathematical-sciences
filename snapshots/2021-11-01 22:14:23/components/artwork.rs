use crate::prelude::*;
use horizontal_strokes::*;
use vertical_boundary::*;

pub struct Artwork {
    rand: Rand,
    vertical_boundary: VerticalBoundary,
    horizontal_strokes: HorizontalStrokes,
}

pub fn new(params: Params) -> Artwork {
    Artwork {
        rand: params.rand,
        vertical_boundary: vertical_boundary::new(params.app),
        horizontal_strokes: horizontal_strokes::new(params.app, &params.container),
    }
}

impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        // circle::render(params);
        // self.vertical_boundary.render(params);
        self.horizontal_strokes.render(params);
    }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}

use crate::prelude::*;
use horizontal_strokes::*;
use vertical_strokes::*;

pub struct Artwork {
    rand: Rand,
    vertical_strokes: VerticalStrokes,
    horizontal_strokes: HorizontalStrokes,
}

pub fn new(params: Params) -> Artwork {
    Artwork {
        rand: params.rand,
        vertical_strokes: vertical_strokes::new(params.app, &params.container, 187),
        horizontal_strokes: horizontal_strokes::new(params.app, &params.container, 189),
    }
}

impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        circle::render(params);
        self.vertical_strokes.render(params);
        self.horizontal_strokes.render(params);
    }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}

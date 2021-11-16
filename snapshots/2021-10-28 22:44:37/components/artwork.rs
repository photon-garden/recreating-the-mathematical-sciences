use crate::prelude::*;
use circle::*;

pub struct Artwork {
    rand: Rand,
    circle: Circle,
}

pub fn new(params: Params) -> Artwork {
    Artwork {
        rand: params.rand,
        circle: Circle {},
    }
}

impl Component for Artwork {
    fn render(&self, params: &mut RenderParams) {
        self.circle.render(params)
    }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}

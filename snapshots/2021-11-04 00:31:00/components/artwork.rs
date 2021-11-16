use crate::prelude::*;

pub struct Artwork {}

pub fn new(params: Params) -> Artwork {
    Artwork {}
}

impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        let time = params.app.time;
        let draw = params.draw;

        let x = time.normalized_cos();
        let y = time.normalized_sin();

        let radius = 0.125;

        let start = pt2(x, y - radius);
        let end = pt2(x, y + radius);

        draw.line()
            .start(start.denormalize(params.container))
            .end(end.denormalize(params.container))
            .color(soft_black());
    }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}

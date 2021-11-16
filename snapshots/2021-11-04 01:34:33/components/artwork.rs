use crate::prelude::*;

pub struct Artwork {}

pub fn new(params: Params) -> Artwork {
    Artwork {}
}

impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        let time = params.app.time;
        let draw = params.draw;

        let radius = 1.0 / 8.0;

        for group_progress in ZERO_TO_ONE.subdivide(16) {
            let group_phase = group_progress.denormalize(0.0, 1.0.turns());
            for individual_progress in ZERO_TO_ONE.subdivide(5) {
                let individual_phase_offset = individual_progress.denormalize(0.5, 1.5);
                let phase = group_phase + individual_phase_offset;

                let x = time.plus(phase).normalized_cos();
                let y = time.plus(phase).times(2.0).normalized_sin();

                let start = pt2(x, y - radius);
                let end = pt2(x, y + radius);

                draw.line()
                    .start(start.denormalize(params.container))
                    .end(end.denormalize(params.container))
                    .color(soft_black());
            }
        }
    }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}

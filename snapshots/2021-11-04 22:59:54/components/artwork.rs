use crate::prelude::*;

pub struct Artwork {}

pub fn new(params: Params) -> Artwork {
    Artwork {}
}

impl Artwork {
    pub fn render(&self, params: &mut RenderParams) {
        let time = params.app.time;
        let draw = params.draw;
        let container = params.container;

        let radius = 0.45 / 2.0;

        let num_groups = 16.0;
        for progress in ZERO_TO_ONE.subdivide(num_groups as u16) {
            let phase_modulation_frequency = progress.denormalize(2.0, 3.0);
            let phase_modulation = time
                .turns()
                .times(phase_modulation_frequency)
                .sin()
                .times(0.1);
            // let phase = progress.plus(phase_modulation);
            let phase = progress.plus(0.0).turns();

            let x_radius = 0.687 / 2.0;
            let x = time
                .plus(phase)
                .normalized_cos()
                .denormalize(0.5 - x_radius, 0.5 + x_radius);

            let y_radius = 0.36 / 2.0;
            let y = time
                .plus(phase)
                .times(2.0)
                .normalized_sin()
                .denormalize(0.5 - y_radius, 0.5 + y_radius);
            // start with 13 clumps, all close together
            // then every 4th group splits into two

            let start = pt2(x, y - radius);
            let end = pt2(x, y + radius);

            draw.line()
                .start(start.denormalize(container))
                .end(end.denormalize(container))
                .color(soft_black());
        }
    }
}

pub struct Params<'a> {
    pub container: Rect,
    pub app: &'a App,
    pub rand: Rand,
}

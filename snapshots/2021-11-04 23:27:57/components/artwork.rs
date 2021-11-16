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

        let num_groups = 16;
        for index in 0..num_groups {
            let progress = index as f32 / num_groups as f32;

            let phase_modulation_frequency = 1.0 + progress;

            let phase_modulation = time
                .times(phase_modulation_frequency)
                .normalized_sin()
                .times(1.0 / num_groups as f32);

            let phase = progress.plus(phase_modulation).turns();

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

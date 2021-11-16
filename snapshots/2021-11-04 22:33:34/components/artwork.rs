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

        let radius = 1.0 / 8.0;

        let num_groups = 7.0;
        for group_progress in ZERO_TO_ONE.subdivide(num_groups as u16) {
            let group_phase = group_progress.denormalize(0.0, 1.0.turns());
            for individual_progress in ZERO_TO_ONE.subdivide(25) {
                let phase_offset_amplitude_modulation = time.times(1.0).normalized_sin();
                let individual_phase_offset_radius = 1.0.turns() / num_groups;
                let individual_phase_offset = individual_progress
                    .denormalize(
                        -individual_phase_offset_radius,
                        individual_phase_offset_radius,
                    )
                    .times(phase_offset_amplitude_modulation);
                let phase = group_phase + individual_phase_offset;

                let x = time.plus(phase).normalized_cos();
                let y = time
                    .plus(phase)
                    .times(2.0)
                    .normalized_sin()
                    .denormalize(0.375, 0.625);

                let start = pt2(x, y - radius);
                let end = pt2(x, y + radius);

                draw.line()
                    .start(start.denormalize(container))
                    .end(end.denormalize(container))
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

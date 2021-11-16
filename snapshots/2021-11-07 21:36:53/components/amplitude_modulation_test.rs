use crate::prelude::*;

pub fn render(params: &mut RenderParams) {
    let time = params.app.time;
    let draw = params.draw;
    let container = params.container;

    let radius = 0.45 / 2.0;

    let num_lines = 16;
    let phase_modulation_amplitudes: Vec<f32> = vec![-0.25, -0.125, 0.125, 0.25];
    let lines_in_group = 4;

    for index in 0..num_lines {
        let progress = index as f32 / num_lines as f32;

        let index_in_group = index % lines_in_group;
        let phase_modulation_frequency = 0.1;
        let phase_modulation_amplitude = phase_modulation_amplitudes.get(index_in_group).unwrap();

        // let phase_modulation = time
        //     .times(phase_modulation_frequency)
        //     .normalized_sin()
        //     .times(*phase_modulation_amplitude);
        let phase_modulation = 0.0;

        let phase = progress.plus(phase_modulation).turns();

        let x_radius = 0.687 / 2.0;
        let x = time
            .plus(phase)
            .normalized_cos()
            .denormalize(0.5 - x_radius, 0.5 + x_radius);

        let y = 0.5;

        let start = pt2(x, y - radius);
        let end = pt2(x, y + radius);

        draw.line()
            .start(start.denormalize(container))
            .end(end.denormalize(container))
            .color(soft_black());
    }
}

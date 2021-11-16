use crate::prelude::*;

// pub fn render(params: &mut RenderParams) {
//     let time = params.app.time;
//     let draw = params.draw;
//     let container = params.container;

//     let radius = 0.45 / 2.0;

//     let num_lines = 12;
//     let lines_in_group = 4;
//     let phase_modulation_amplitudes: Vec<f32> = vec![-0.25, -0.125, 0.125, 0.25];

//     for index in 0..num_lines {
//         let progress = index as f32 / num_lines as f32;

//         let index_in_group = index % lines_in_group;

//         let phase_modulation_frequency = 0.1;
//         let phase_modulation_amplitude = phase_modulation_amplitudes.get(index_in_group).unwrap();

//         let phase_modulation = time
//             .times(phase_modulation_frequency)
//             .normalized_sin()
//             .times(*phase_modulation_amplitude);

//         let phase = progress.plus(phase_modulation).turns();

//         let x_radius = 0.687 / 2.0;
//         let x = time
//             .plus(phase)
//             .normalized_cos()
//             .denormalize(0.5 - x_radius, 0.5 + x_radius);

//         let y_radius = 0.36 / 2.0;
//         let y = time
//             .plus(phase)
//             .times(2.0)
//             .normalized_sin()
//             .denormalize(0.5 - y_radius, 0.5 + y_radius);
//         // start with 13 clumps, all close together
//         // then every 4th group splits into two

//         let start = pt2(x, y - radius);
//         let end = pt2(x, y + radius);

//         draw.line()
//             .start(start.denormalize(container))
//             .end(end.denormalize(container))
//             .color(soft_black());
//     }
// }

pub fn render(params: &mut RenderParams) {
    let time = params.app.time;
    let draw = params.draw;
    let container = params.container;
    let model = params.model;
    let groups = model.line_group_sets.get(model.line_group_index).unwrap();

    let radius = 0.45 / 2.0;

    let num_lines: usize = groups.iter().map(|group| group.size).sum();
    if num_lines != 16 {
        panic!("Wrong number of lines.");
    }

    let mut index = 0;
    let mut num_lines_in_groups_already_processed = 0;
    for group in groups {
        let center_index_offset = if group.size.is_even() { 0.5 } else { 1.0 };

        let center_index_for_group = (group.size as f32)
            .divided_by(2.0)
            .floor()
            .plus(center_index_offset);
        let center_index =
            (num_lines_in_groups_already_processed) as f32 + center_index_for_group - 1.0;

        let center_phase = center_index / num_lines as f32;

        let stroke_weight = container.lerp_w(0.00949598);

        for _index_in_group in 0..group.size {
            let progress = index as f32 / num_lines as f32;

            let phase_modulation_min = 0.0;
            let phase_modulation_max = center_phase - progress;

            let phase_modulation = model
                .phase_modulation_sine
                .denormalize(phase_modulation_min, phase_modulation_max);

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

            let start = pt2(x, y - radius);
            let end = pt2(x, y + radius);

            draw.line()
                .start(start.denormalize(container))
                .end(end.denormalize(container))
                .stroke_weight(stroke_weight)
                .color(whitney_white());

            index += 1;
        }

        num_lines_in_groups_already_processed += group.size;
    }
}

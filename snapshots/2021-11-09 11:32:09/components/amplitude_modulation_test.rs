use crate::prelude::*;

pub fn render(params: &mut RenderParams) {
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

        for _index_in_group in 0..group.size {
            let progress = index as f32 / num_lines as f32;

            let phase_modulation_min = 0.0;
            let phase_modulation_max = center_phase - progress;

            let phase_modulation = model
                .phase_modulation_sine
                .denormalize(phase_modulation_min, phase_modulation_max);

            let phase = progress.plus(phase_modulation);

            let x = phase;
            let y = 0.5;

            let start = pt2(x, y - radius);
            let end = pt2(x, y + radius);

            draw.line()
                .start(start.denormalize(container))
                .end(end.denormalize(container))
                .color(soft_white());

            index += 1;
        }

        num_lines_in_groups_already_processed += group.size;
    }
}

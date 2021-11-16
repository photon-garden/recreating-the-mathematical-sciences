use crate::prelude::*;

pub fn render(params: &mut RenderParams) {
    let time = params.app.time;
    let draw = params.draw;
    let container = params.container;

    let radius = 0.45 / 2.0;

    let groups = vec![
        LineGroup { size: 6 },
        LineGroup { size: 5 },
        LineGroup { size: 5 },
    ];
    let num_lines: usize = groups.iter().map(|group| group.size).sum();
    let max_phase_modulation_amplitude = 1.0 / num_lines as f32;
    let phase_modulation_frequency = 1.0;
    let x_radius = 0.687 / 2.0;

    let mut index = 0;
    for group in groups {
        let phase_modulation_amplitudes = group.centered_indexes().normalize().denormalize(
            -max_phase_modulation_amplitude,
            max_phase_modulation_amplitude,
        );

        for index_in_group in 0..group.size {
            let progress = index as f32 / num_lines as f32;
            let phase_modulation_amplitude =
                phase_modulation_amplitudes.get(index_in_group).unwrap();

            let phase_modulation = time
                .times(phase_modulation_frequency)
                .normalized_sin()
                .times(*phase_modulation_amplitude);

            let phase = progress.plus(phase_modulation);

            let x = phase.denormalize(0.5 - x_radius, 0.5 + x_radius);

            let y = 0.5;

            let start = pt2(x, y - radius);
            let end = pt2(x, y + radius);

            draw.line()
                .start(start.denormalize(container))
                .end(end.denormalize(container))
                .color(soft_black());

            index += 1;
        }
    }
}

// If you have a length of 5, it converts indexes as follows:
//
// 0 becomes -2
// 1 becomes -1
// 2 becomes 0
// 3 becomes 1
// 4 becomes 2
//
// For an array with a length of 4:
//
// 0 becomes -1.5
// 1 becomes -0.5
// 2 becomes 0.5
// 3 becomes 1.5
//
// Useful for computations where you have an array of elements
// and you need to center them.
struct LineGroup {
    size: usize,
}

impl LineGroup {
    fn centered_indexes(&self) -> Vec<f32> {
        (0..self.size)
            .map(|index| self.center_index(index))
            .collect()
    }

    fn center_index(&self, index: usize) -> f32 {
        let middle_index = if self.size.is_odd() {
            // If length is 5, we're iterating over indexes
            // from 0 to 4. The middle element of the array
            // will be at index 2.
            (self.size as f32 / 2.0).floor()
        } else {
            // If we're building an array with even length,
            // it doesn't actually have a middle element.
            // But if you think of it as having an *imaginary*
            // middle element halfway between the two most central
            // elements, the math works out.
            //
            // There's probably a more elegant way to think about this.
            (self.size as f32 / 2.0) - 0.5
        };

        index as f32 - middle_index
    }
}

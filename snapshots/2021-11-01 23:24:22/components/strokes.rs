use crate::prelude::*;

pub type Line = [Point2; 2];

pub fn render(lines: &Vec<Line>, params: &RenderParams) {
    let draw = params.draw;
    let stroke_weight = params.container.lerp_w(0.003);

    for line in lines.iter() {
        draw.line()
            .start(line[0])
            .end(line[1])
            .weight(stroke_weight)
            .color(soft_black());
    }
}

use crate::prelude::*;

pub type Line = [Point2; 2];

pub struct Strokes {
    lines: Vec<Line>,
}

pub fn new(lines: Vec<Line>) -> Strokes {
    Strokes { lines }
}

impl Strokes {
    pub fn render(&self, params: &RenderParams) {
        let draw = params.draw;
        let stroke_weight = params.container.lerp_w(0.001);

        for line in self.lines.iter() {
            draw.line()
                .start(line[0])
                .end(line[1])
                .weight(stroke_weight)
                .color(soft_black());
        }
    }
}

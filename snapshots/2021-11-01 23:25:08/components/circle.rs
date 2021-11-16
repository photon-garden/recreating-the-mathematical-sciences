use crate::prelude::*;

pub fn render(params: &mut RenderParams) {
    let container = params.container;
    let draw = params.draw;

    let (normalized_center, normalized_radius) = normalized_center_and_radius();

    let center = container.denormalize_xy(&normalized_center);
    let radius = container.lerp_w(normalized_radius);
    let stroke_weight = container.lerp_w(0.0015);

    draw.ellipse()
        .xy(center)
        .radius(radius)
        .color(soft_black())
        .stroke_weight(stroke_weight)
        .no_fill();
}

pub fn normalized_center_and_radius() -> (Point2, f32) {
    let center = pt2(0.5, 0.5);
    let radius = 0.968 / 2.0;

    (center, radius)
}

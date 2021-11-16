use crate::prelude::*;
use derive_builder::Builder;

#[derive(Builder, Debug, Clone, Default)]
#[builder(default)]
pub struct Circle {}

impl Component for Circle {
    fn render(&self, params: &mut RenderParams) {
        let container = params.container;
        let draw = params.draw;

        let center = container.xy();
        let radius = container.lerp_w(0.965);

        draw.ellipse().xy(center).radius(radius);
    }
}

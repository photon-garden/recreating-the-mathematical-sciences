#![allow(dead_code)]

mod colors;
mod components;
mod crates;
mod extensions;
mod grid;
mod line_group_sets;
mod prelude;
mod snapshot;
mod svg;
mod window_dimensions;

use std::env;

use prelude::*;

fn main() {
    nannou::app(start).update(update).exit(snapshot::exit).run();
}

fn start(app: &App) -> Model {
    let window_dimensions = window_dimensions::load();

    let is_animated = env::args().any(|argument| argument == "--animate");
    let is_still = !is_animated;

    if is_still {
        app.set_loop_mode(LoopMode::loop_ntimes(1));
    }

    let mut window_builder = app
        .new_window()
        .view(draw)
        .size(window_dimensions.width, window_dimensions.height)
        // .max_size(10_000, 10_000)
        // .maximized(true)
        .decorations(false);

    if is_animated {
        window_builder = window_builder.key_released(capture_frame_on_s);
    }

    window_builder.build().unwrap();

    let mut snapshot = snapshot::save();
    if is_still {
        snapshot.capture_frame(app);
    }

    let rand = snapshot.get_rand();

    let container_wh = vec2(
        window_dimensions.container_width,
        window_dimensions.container_height,
    );

    let container = Rect::from_xy_wh(Point2::zero(), container_wh);

    let root_component_params = components::Params {
        app,
        rand,
        container,
    };

    Model {
        snapshot,
        root_component: components::new(root_component_params),
        container,
        line_group_sets: line_group_sets::get(),
        line_group_index: 0,
        phase_modulation_sine: 0.0,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let phase_modulation_frequency = 1.0;
    model.phase_modulation_sine = app.time.times(phase_modulation_frequency).normalized_sin();

    let last_line_group_index = model.line_group_sets.len() - 1;

    dbg!(app.elapsed_frames());
    dbg!(model.phase_modulation_sine);
    dbg!(last_line_group_index);

    if app.elapsed_frames() > 0
        && model.phase_modulation_sine.close_to(0.0)
        && model.line_group_index < last_line_group_index
    {
        model.line_group_index += 1;
        dbg!("Updating line group index.");
    }
}

pub fn draw(app: &App, model: &Model, frame: Frame) {
    let mut rand = model.snapshot.get_rand();
    let draw = app.draw();

    draw.background().color(soft_white());

    let mut params = RenderParams {
        app,
        model,
        rand: &mut rand,
        draw: &draw,
        container: &model.container,
    };

    model.root_component.render(&mut params);

    draw.to_frame(app, &frame).unwrap();
}

fn capture_frame_on_s(app: &App, model: &mut Model, key: Key) {
    if key == Key::S {
        model.snapshot.capture_frame(app);
    }
}

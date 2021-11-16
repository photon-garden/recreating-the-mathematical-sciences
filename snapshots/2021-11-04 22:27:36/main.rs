#![allow(dead_code)]

mod colors;
mod components;
mod config;
mod crates;
mod extensions;
mod grid;
mod prelude;
mod snapshot;
mod svg;

use std::env;

use prelude::*;

fn main() {
    nannou::app(start).update(update).exit(snapshot::exit).run();
}

fn start(app: &App) -> Model {
    let config_params = config::load();

    let is_animated = env::args().any(|argument| argument == "--animate");
    let is_still = !is_animated;

    if is_still {
        app.set_loop_mode(LoopMode::loop_ntimes(1));
    }

    let mut window_builder = app
        .new_window()
        .view(draw)
        .size(config_params.width, config_params.height)
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
        config_params.container_width,
        config_params.container_height,
    );

    let container = Rect::from_xy_wh(Point2::zero(), container_wh);
    dbg!(app.main_window().rect());
    dbg!(container);

    let artwork_params = components::artwork::Params {
        app,
        rand,
        container,
    };

    Model {
        snapshot,
        root_component: components::artwork::new(artwork_params),
        container,
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

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

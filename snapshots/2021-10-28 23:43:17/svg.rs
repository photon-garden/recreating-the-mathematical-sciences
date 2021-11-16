use crate::prelude::*;
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;

pub fn parse_path(app: &App, svg_file_name: &str) -> Vec<Point2> {
    let path_to_svg_file = app.assets_path().unwrap().join(svg_file_name);

    let mut content = String::new();

    let mut points: Vec<Point2> = vec![];

    let path = vec![];
    let mut current_point = pt2(0.0, 0.0);

    for event in svg::open(path_to_svg_file, &mut content).unwrap() {
        if let Event::Tag(_path, _, attributes) = event {
            println!("Got an event.");
            let data = attributes.get("d").unwrap();
            let data = Data::parse(data).unwrap();
            for command in data.iter() {
                match command {
                    Command::Move(position, params) => {
                        println!("Move!");
                        dbg!(position, params);
                    }
                    Command::Line(position, params) => {
                        println!("Line!");
                        dbg!(position, params);
                    }
                    _ => {}
                }
            }
        }
    }

    path
}

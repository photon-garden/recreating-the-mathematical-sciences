use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;

pub fn parse_path(path_to_svg_file: &str) -> Vec<Point2> {
    let mut content = String::new();

    let mut points = vec![];

    let path = vec![];
    let mut current_point = pt2(0.0, 0.0);

    for event in svg::open(path_to_svg_file, &mut content).unwrap() {
        match event {
            Event::Tag(Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                for command in data.iter() {
                    match command {
                        &Command::Move(position, params) => {
                            dbg!(position, params);
                        }
                        &Command::Line(..) => {
                            println!("Line!");
                            dbg!(position, params);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

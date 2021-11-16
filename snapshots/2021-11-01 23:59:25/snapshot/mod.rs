mod friendly_words;
mod image;
mod manifest;
pub mod rand;
mod seed;
mod source_code;

use rand::Rand;
use seed::Seed;

pub struct Snapshot {
    pub did_capture_frames: bool,
    pub seed: u64,
    frame_number: u64,
    created_at: String,
    image_name_randomizer: Rand,
}

impl Snapshot {
    fn create() -> Snapshot {
        let seed = Seed::load();
        seed.save_to_file();

        let created_at = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let new_snapshot = Snapshot {
            seed: seed.value,
            did_capture_frames: false,
            frame_number: 0,
            created_at,
            image_name_randomizer: Rand::from_seed(seed.value),
        };

        source_code::save_current_version(&new_snapshot.source_code_folder_name());
        seed.clean_up_file();

        new_snapshot
    }

    fn image_name(&mut self) -> String {
        let predicates = friendly_words::predicates();
        let objects = friendly_words::objects();

        let first_predicate = self.image_name_randomizer.element(&predicates);
        let second_predicate = self.image_name_randomizer.element(&predicates);

        let object = self.image_name_randomizer.element(&objects);

        format!("{} {} {}", first_predicate, second_predicate, object)
    }

    fn source_code_folder_name(&self) -> String {
        self.created_at.clone()
    }

    pub fn get_rand(&self) -> Rand {
        Rand::from_seed(self.seed)
    }

    pub fn capture_frame(&mut self, app: &nannou::prelude::App) {
        image::capture_frame(self, app);

        self.did_capture_frames = true;
        self.frame_number += 1;
    }
}

pub fn save() -> Snapshot {
    Snapshot::create()
}

pub fn exit(app: &nannou::prelude::App, model: crate::prelude::Model) {
    if model.snapshot.did_capture_frames {
        image::clean_up(app, &model.snapshot);
    }
}

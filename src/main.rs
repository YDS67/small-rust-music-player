#![windows_subsystem = "windows"]

use std::sync::{Arc, Mutex};
use std::thread;
use miniquad::{self, conf::Platform, conf::Conf};

mod audio;
mod settings;
mod assets;
mod input;
mod mesh;
mod text;
mod shaders;
mod stage;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Player".to_owned(),
        window_width: settings::WIDTH0,
        window_height: settings::HEIGHT0,
        window_resizable: true,
        platform: Platform::default(),
        ..Default::default()
    };
    conf.platform.swap_interval = Some(1);
    conf
}


#[derive(Debug, Clone)]
pub struct State {
    pub play: bool,
    pub skip: bool,
    pub repeat: bool,
    pub dir_name: String,
    pub file_num: usize,
    pub file_name: String,
    pub file_ext: String,
    pub message: String,
    pub sample_stats: [i16; settings::SAMPLES],
}

fn main() {
    let state = State {
        play: true,
        skip: false,
        repeat: false,
        file_num: 0,
        dir_name: format!("Directory not found"),
        file_name: format!("File not found"),
        file_ext: format!("Unknown"),
        message: format!("***"),
        sample_stats: [0; settings::SAMPLES],
    };

    let state = Arc::new(Mutex::new(state));
    let state_player = Arc::clone(&state);

    thread::spawn(|| audio::playback(state_player));

    miniquad::start(window_conf(), move || {Box::new(stage::Stage::new(state))});
}

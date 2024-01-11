use miniquad::*;
use std::thread::sleep;
use std::time::Duration;

use crate::settings;

pub struct TimeState {
    pub last_frame: f64,
    pub frame_time: f64,
    pub fps: i32,
}

impl TimeState {
    pub fn init() -> TimeState {
        TimeState {
            last_frame: date::now(),
            frame_time: 1.0 / 60.0,
            fps: 60,
        }
    }

    pub fn frame_time(&mut self, settings: &mut settings::Settings) {
        self.frame_time = date::now() - self.last_frame;
        if self.frame_time < settings::FT_DESIRED {
            sleep(Duration::from_secs_f64(
                settings::FT_DESIRED - self.frame_time,
            ));
        }
        self.frame_time = date::now() - self.last_frame;
        settings.delta_time = self.frame_time as f32;
        self.fps = (1. / self.frame_time).floor() as i32;

        settings.player_speed = 12.0 * settings.delta_time;
    }
}

pub struct KeysState {
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub q: bool,
    pub e: bool,
    pub k: bool,
    pub l: bool,
    pub f: bool,
    pub m: bool,
    pub esc: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub space: bool,
    pub enter: bool,
}

impl KeysState {
    pub fn read_key(&mut self, keycode: KeyCode, state: bool) {
        match keycode {
            KeyCode::W => self.w = state,
            KeyCode::S => self.s = state,
            KeyCode::A => self.a = state,
            KeyCode::D => self.d = state,
            KeyCode::Left => self.left = state,
            KeyCode::Right => self.right = state,
            KeyCode::Up => self.up = state,
            KeyCode::Down => self.down = state,
            KeyCode::Space => self.space = state,
            KeyCode::Escape => self.esc = state,
            KeyCode::Enter => self.enter = state,
            KeyCode::K => self.k = state,
            KeyCode::L => self.l = state,
            KeyCode::Q => self.q = state,
            KeyCode::E => self.e = state,
            KeyCode::F => self.f = state,
            KeyCode::M => self.m = state,
            _ => {},
        }
    }
}

pub struct InputState {
    pub keys: KeysState,
    pub apply_change: bool,
}

impl InputState {
    pub fn init() -> InputState {
        InputState {
            keys: KeysState {
                w: false,
                a: false,
                s: false,
                d: false,
                q: false,
                e: false,
                k: false,
                l: false,
                f: false,
                m: false,
                left: false,
                right: false,
                up: false,
                down: false,
                space: false,
                enter: false,
                esc: false,
            },
            apply_change: false,
        }
    }
}

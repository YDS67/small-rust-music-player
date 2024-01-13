pub const WIDTH0: i32 = 512;
pub const HEIGHT0: i32 = 512;
pub const WIDTH: u32 = 512;
pub const HEIGHT: u32 = 512;
pub const FT_DESIRED: f64 = 1.0/120.0;
pub const SAMPLING_TIME: f64 = 1.0/20.0;
pub const MAX_QUADS_OVERLAY: usize = 1000;
pub const MAX_VERTICES_OVERLAY: usize = MAX_QUADS_OVERLAY*4;
pub const MAX_INDICES_OVERLAY: usize = MAX_QUADS_OVERLAY*6;
pub const SAMPLES: usize = 1024;
pub const _AVERAGE_FREQ: usize = 16;
pub const BINS: usize = 16;
pub const BIN_SAMPLES: usize = SAMPLES/BINS;
pub const AVERAGE_TIME: usize = 1;
pub const MAX_VERTICES_VISUALS: usize = BINS*4;
pub const MAX_INDICES_VISUALS: usize = BINS*6;
pub const MAX_QUADS_GUI: usize = 1000;
pub const MAX_VERTICES_GUI: usize = MAX_QUADS_GUI*4;
pub const MAX_INDICES_GUI: usize = MAX_QUADS_GUI*6;
pub const _CLR1: (f32, f32, f32, f32) = (0.5294118, 0.8078431, 0.9215686, 1.0000000); // sky blue
pub const CLR2: (f32, f32, f32, f32) = (0.14117647, 0.07843137, 0.13333333, 1.0); // dark purple
pub const _CLR3: (f32, f32, f32, f32) = (0.8, 0.0, 0.2, 1.0); // nice red
pub const CLR4: (f32, f32, f32, f32) = (0.1568627, 0.1568627, 0.1568627, 1.0); // almost black
pub const _CLR5: (f32, f32, f32, f32) = (0.9960784, 0.7607843, 0.5568627, 1.0); // pale yellow
pub const CLR6: (f32, f32, f32, f32) = (0.1843137, 0.2666667, 0.4627451, 1.0000000); // gray-blue
pub const _CLR7: (f32, f32, f32, f32) = (0.0, 0.2745098, 0.6666667, 1.0000000); // blue
pub const CLR8: (f32, f32, f32, f32) = (0.85, 0.85, 0.85, 1.0000000); // almost white
pub const _CLR9: (f32, f32, f32, f32) = (0.5105882, 0.5600000, 0.6776471, 1.0000000); // pale blue

pub struct Settings {
    pub screen_width: i32,
    pub screen_height: i32,
    pub full_screen: bool,
    pub screen_width_f: f32,
    pub screen_height_f: f32,
    pub screen_aspect: f32,
}

impl Settings {
    pub fn init() -> Settings {
        let screen_width = WIDTH0;
        let screen_height = HEIGHT0;
        let full_screen = false;
        let screen_width_f = screen_width as f32;
        let screen_height_f = screen_height as f32;
        let screen_aspect = screen_width_f/screen_height_f;
        Settings {
            screen_width,
            screen_height,
            full_screen,
            screen_width_f,
            screen_height_f,
            screen_aspect,
        }
    }

    pub fn screen_change(&mut self, screen_width: f32, screen_height: f32) {
        self.screen_width = screen_width as i32;
        self.screen_height = screen_height as i32;
        self.screen_width_f = screen_width;
        self.screen_height_f = screen_height;
        self.screen_aspect = screen_width/screen_height;
    }
}

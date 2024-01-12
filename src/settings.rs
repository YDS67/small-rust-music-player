pub const PI: f32 = 3.1415926538;
pub const MAPSIZE: usize = 256;
pub const WIDTH0: i32 = 500;
pub const HEIGHT0: i32 = 250;
pub const WIDTH: u32 = 500;
pub const HEIGHT: u32 = 250;
pub const FT_DESIRED: f64 = 0.01666666666667;
pub const MAX_QUADS_OVERLAY: usize = 1000;
pub const MAX_VERTICES_OVERLAY: usize = MAX_QUADS_OVERLAY*4;
pub const MAX_INDICES_OVERLAY: usize = MAX_QUADS_OVERLAY*6;
pub const MAX_QUADS_GUI: usize = 1000;
pub const MAX_VERTICES_GUI: usize = MAX_QUADS_GUI*4;
pub const MAX_INDICES_GUI: usize = MAX_QUADS_GUI*6;
pub const CLR1: (f32, f32, f32, f32) = (0.5294118, 0.8078431, 0.9215686, 1.0000000); // sky blue
pub const CLR2: (f32, f32, f32, f32) = (0.14117647, 0.07843137, 0.13333333, 1.0); // dark purple
pub const CLR3: (f32, f32, f32, f32) = (0.8, 0.0, 0.2, 1.0); // nice red
pub const CLR4: (f32, f32, f32, f32) = (0.1568627, 0.1568627, 0.1568627, 1.0); // almost black
pub const CLR5: (f32, f32, f32, f32) = (0.9960784, 0.7607843, 0.5568627, 1.0); // pale yellow
pub const CLR6: (f32, f32, f32, f32) = (0.1843137, 0.2666667, 0.4627451, 1.0000000); // gray-blue
pub const CLR7: (f32, f32, f32, f32) = (0.0, 0.2745098, 0.6666667, 1.0000000); // blue
pub const CLR8: (f32, f32, f32, f32) = (0.85, 0.85, 0.85, 1.0000000); // almost white
pub const CLR9: (f32, f32, f32, f32) = (0.5105882, 0.5600000, 0.6776471, 1.0000000); // pale blue

pub struct Settings {
    pub screen_width: i32,
    pub screen_height: i32,
    pub full_screen: bool,
    pub draw_map: bool,
    pub draw_menu: bool,
    pub screen_width_f: f32,
    pub screen_height_f: f32,
    pub screen_aspect: f32,
    pub player_height: f32,
    pub tile_screen_size: f32,
    pub map_size_f: f32,
    pub map_offset_x: f32,
    pub map_offset_y: f32,
    pub player_x0: f32,
    pub player_y0: f32,
    pub player_a0: f32,
    pub player_b0: f32,
    pub fov_xy: f32,
    pub fov_z: f32,
    pub delta_time: f32,
    pub player_speed: f32,
    pub player_radius: f32,
    pub draw_max_dist: f32,
    pub draw_min_dist: usize,
    pub light_dist: f32,
    pub draw_rays_num: usize,
    pub mouse_sensitivity: f32,
    pub music_playing: bool,
}

impl Settings {
    pub fn init() -> Settings {
        let screen_width = WIDTH0;
        let screen_height = HEIGHT0;
        let full_screen = false;
        let draw_map = false;
        let draw_menu = false;
        let screen_width_f = screen_width as f32;
        let screen_height_f = screen_height as f32;
        let screen_aspect = screen_width_f/screen_height_f;
        let player_height = 0.5;
        let tile_screen_size = 1.5;
        let map_size_f = 256.0;
        let map_offset_x = 20.0;
        let map_offset_y = screen_height_f - tile_screen_size * (MAPSIZE as f32) - 20.0;
        let player_x0 = MAPSIZE as f32 / 2.0;
        let player_y0 = 4.5;
        let player_a0 = 1.55;
        let player_b0 = 0.0;
        let fov_xy = PI / 4.0;
        let fov_z = fov_xy / screen_aspect;
        let delta_time = 1.0/60.0;
        let player_speed = 12.0*delta_time;
        let player_radius = 0.5;
        let draw_max_dist = 100.0;
        let draw_min_dist = 3*3;
        let light_dist = 5.0;
        let draw_rays_num = 1920;
        let mouse_sensitivity = 10.0;
        Settings {
            screen_width,
            screen_height,
            full_screen,
            draw_map,
            draw_menu,
            screen_width_f,
            screen_height_f,
            screen_aspect,
            player_height,
            tile_screen_size,
            map_size_f,
            map_offset_x,
            map_offset_y,
            player_x0,
            player_y0,
            player_a0,
            player_b0,
            fov_xy,
            fov_z,
            delta_time,
            player_speed,
            player_radius,
            draw_max_dist,
            draw_min_dist,
            light_dist,
            draw_rays_num,
            mouse_sensitivity,
            music_playing: true,
        }
    }

    pub fn screen_change(&mut self, screen_width: f32, screen_height: f32) {
        self.screen_width = screen_width as i32;
        self.screen_height = screen_height as i32;
        self.screen_width_f = screen_width;
        self.screen_height_f = screen_height;
        self.screen_aspect = screen_width/screen_height;
        self.map_offset_y = screen_height - self.tile_screen_size * (MAPSIZE as f32) - 20.0;
    }
}

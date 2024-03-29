use crate::{mesh::TextureUV, settings};
use std::char;

pub const WIDTH: f32 = 12.0;
pub const HEIGHT: f32 = 20.0;
pub const HEIGHT_CAP: f32 = 20.0;
const X1: f32 = 3.0;
const Y1: f32 = 2.0;
const Y2: f32 = Y1+HEIGHT_CAP;
const Y3: f32 = Y2+HEIGHT_CAP;
const Y4: f32 = Y3+HEIGHT;
const Y5: f32 = Y4+HEIGHT;
const Y6: f32 = Y5+HEIGHT;
const TEXWIDTH: f32 = 256.0;
const TEXHEIGHT: f32 = 256.0;

pub struct Overlay {
    pub lines: Vec<String>,
    pub line_width: Vec<f32>,
    pub line_x: Vec<f32>,
    pub line_y: Vec<f32>,
    pub font_col: (f32, f32, f32, f32),
    pub x0: f32,
    pub y0: f32,
    pub scale: f32,
    pub line_height: f32,
}

impl Overlay {
    pub fn new_from(lines: Vec<&str>) -> Overlay {
        let x0 = 20.0;
        let y0 = 20.0;
        let scale = 1.0;
        let line_height = HEIGHT*scale;
        let mut lines1 = Vec::new();
        let mut line_width = Vec::new();
        let mut line_x = Vec::new();
        let mut line_y = Vec::new();
        for l in 0..lines.len() {
            lines1.push(lines[l].to_string());
            let letters: Vec<char> = lines[l].chars().collect();
            line_width.push(letters.len() as f32 * WIDTH * scale);
            line_x.push(x0*scale);
            line_y.push(y0*scale + (l as f32)*line_height)
        }
        let overlay = Overlay {
            lines: lines1,
            line_width,
            line_x,
            line_y,
            font_col: settings::CLR8,
            x0,
            y0,
            scale,
            line_height,
        };
        overlay
    }
}

pub struct GUI {
    pub lines: Vec<String>,
    pub line_width: Vec<f32>,
    pub line_x: Vec<f32>,
    pub line_y: Vec<f32>,
    pub line_active: Vec<i32>,
    pub font_col: (f32, f32, f32, f32),
    pub act_col: (f32, f32, f32, f32),
    pub act_no: usize,
    pub x0: f32,
    pub y0: f32,
    pub max_width: f32,
    pub scale: f32,
    pub line_height: f32,
    pub show: bool,
}

impl GUI {
    pub fn new_from(lines: Vec<&str>, width: f32, height: f32) -> GUI {
        let scale = 1.0;
        let line_height = HEIGHT*scale;
        let x0 = width*0.5;
        let y0 = (height - (lines.len() as f32 - 1.0)*line_height)*0.3/scale;
        let mut lines1 = Vec::new();
        let mut line_width = Vec::new();
        let mut line_x = Vec::new();
        let mut line_y = Vec::new();
        let mut line_active = Vec::new();
        for l in 0..lines.len() {
            lines1.push(lines[l].to_string());
            let letters: Vec<char> = lines[l].chars().collect();
            line_width.push(letters.len() as f32 * WIDTH * scale);
            line_x.push(x0*scale);
            line_y.push(y0*scale + (l as f32)*line_height);
            line_active.push(0);
        }
        let max_width = vec_max(&line_width);
        let mut gui = GUI {
            lines: lines1,
            line_width,
            line_x,
            line_y,
            line_active,
            font_col: settings::CLR2,
            act_col: settings::CLR6,
            act_no: 0,
            x0,
            y0,
            max_width,
            scale,
            line_height,
            show: true,
        };
        gui.center();
        gui
    }

    fn center(&mut self) {
        self.x0 = self.x0 - 0.5*self.max_width;
        for l in 0..self.lines.len() {
            self.line_x[l] = self.x0 + 0.5*self.max_width - 0.5*self.line_width[l]
        }
    }

}

fn char_to_uv(c: char) -> TextureUV {
    let tex;

    match c {
        'A' => tex = TextureUV{u1: X1, u2: X1+WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'А' => tex = TextureUV{u1: X1, u2: X1+WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Я' => tex = TextureUV{u1: X1, u2: X1+WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'B' => tex = TextureUV{u1: X1+WIDTH, u2: X1+2.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Б' => tex = TextureUV{u1: X1+WIDTH, u2: X1+2.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'C' => tex = TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Ц' => tex = TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Ч' => tex = TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'D' => tex = TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Д' => tex = TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'E' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Е' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Ё' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Э' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'F' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Ф' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'G' => tex = TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Г' => tex = TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'H' => tex = TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Х' => tex = TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'I' => tex = TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'И' => tex = TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'J' => tex = TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Ж' => tex = TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'K' => tex = TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'К' => tex = TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'L' => tex = TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'Л' => tex = TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'M' => tex = TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        'М' => tex = TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y1, v2: Y1+HEIGHT_CAP},
        '=' => tex = TextureUV{u1: 0.0, u2: 128.0, v1: 128.0, v2: 256.0},
        'N' => tex = TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'Н' => tex = TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'O' => tex = TextureUV{u1: X1+1.0*WIDTH, u2: X1+2.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'О' => tex = TextureUV{u1: X1+1.0*WIDTH, u2: X1+2.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'P' => tex = TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'П' => tex = TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'Q' => tex = TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'R' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'Р' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'р' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y4, v2: Y4+HEIGHT_CAP},
        'S' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'С' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'Ш' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'Щ' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'с' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y4, v2: Y4+HEIGHT_CAP},
        'T' => tex = TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'Т' => tex = TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'т' => tex = TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y4, v2: Y4+HEIGHT_CAP},
        'U' => tex = TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'У' => tex = TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'у' => tex = TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y4, v2: Y4+HEIGHT_CAP},
        'Ю' => tex = TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'ю' => tex = TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y4, v2: Y4+HEIGHT_CAP},
        'V' => tex = TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'В' => tex = TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'в' => tex = TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y4, v2: Y4+HEIGHT_CAP},
        'W' => tex = TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'X' => tex = TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'Y' => tex = TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'Й' => tex = TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'Ы' => tex = TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'й' => tex = TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y4, v2: Y4+HEIGHT_CAP},
        'ы' => tex = TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y4, v2: Y4+HEIGHT_CAP},
        'Z' => tex = TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'З' => tex = TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y2, v2: Y2+HEIGHT_CAP},
        'з' => tex = TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y4, v2: Y4+HEIGHT_CAP},
        'a' => tex = TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'а' => tex = TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'я' => tex = TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'b' => tex = TextureUV{u1: X1+1.0*WIDTH, u2: X1+2.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'б' => tex = TextureUV{u1: X1+1.0*WIDTH, u2: X1+2.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'c' => tex = TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'ц' => tex = TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'ч' => tex = TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'd' => tex = TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'д' => tex = TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'e' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'е' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'ё' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'э' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'f' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'ф' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'g' => tex = TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'г' => tex = TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'h' => tex = TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'х' => tex = TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'i' => tex = TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'и' => tex = TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'j' => tex = TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'ж' => tex = TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'k' => tex = TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'к' => tex = TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'l' => tex = TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'л' => tex = TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'm' => tex = TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'м' => tex = TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y3, v2: Y3+HEIGHT},
        'n' => tex = TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'н' => tex = TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'o' => tex = TextureUV{u1: X1+1.0*WIDTH, u2: X1+2.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'о' => tex = TextureUV{u1: X1+1.0*WIDTH, u2: X1+2.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'p' => tex = TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'п' => tex = TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'q' => tex = TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'r' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        's' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'ш' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'щ' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        't' => tex = TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'u' => tex = TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'v' => tex = TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'w' => tex = TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'x' => tex = TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'y' => tex = TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'z' => tex = TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        '*' => tex = TextureUV{u1: X1+13.0*WIDTH, u2: X1+14.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        '“' => tex = TextureUV{u1: X1+14.0*WIDTH, u2: X1+15.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        '”' => tex = TextureUV{u1: X1+15.0*WIDTH, u2: X1+16.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        '!' => tex = TextureUV{u1: X1+16.0*WIDTH, u2: X1+17.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        '’' => tex = TextureUV{u1: X1+17.0*WIDTH, u2: X1+18.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'Ь' => tex = TextureUV{u1: X1+17.0*WIDTH, u2: X1+18.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'Ъ' => tex = TextureUV{u1: X1+17.0*WIDTH, u2: X1+18.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'ь' => tex = TextureUV{u1: X1+17.0*WIDTH, u2: X1+18.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        'ъ' => tex = TextureUV{u1: X1+17.0*WIDTH, u2: X1+18.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        '\'' => tex = TextureUV{u1: X1+17.0*WIDTH, u2: X1+18.0*WIDTH, v1: Y4, v2: Y4+HEIGHT},
        '●' => tex = TextureUV{u1: X1+18.0*WIDTH+1.0, u2: X1+19.0*WIDTH-1.0, v1: 68.0, v2: 78.0},
        '1' => tex = TextureUV{u1: X1+0.0*WIDTH, u2: X1+1.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '2' => tex = TextureUV{u1: X1+1.0*WIDTH, u2: X1+2.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '3' => tex = TextureUV{u1: X1+2.0*WIDTH, u2: X1+3.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '4' => tex = TextureUV{u1: X1+3.0*WIDTH, u2: X1+4.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '5' => tex = TextureUV{u1: X1+4.0*WIDTH, u2: X1+5.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '6' => tex = TextureUV{u1: X1+5.0*WIDTH, u2: X1+6.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '7' => tex = TextureUV{u1: X1+6.0*WIDTH, u2: X1+7.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '8' => tex = TextureUV{u1: X1+7.0*WIDTH, u2: X1+8.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '9' => tex = TextureUV{u1: X1+8.0*WIDTH, u2: X1+9.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '0' => tex = TextureUV{u1: X1+9.0*WIDTH, u2: X1+10.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        ' ' => tex = TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '.' => tex = TextureUV{u1: X1+11.0*WIDTH, u2: X1+12.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        ':' => tex = TextureUV{u1: X1+12.0*WIDTH, u2: X1+13.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        ',' => tex = TextureUV{u1: X1+13.0*WIDTH, u2: X1+14.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '?' => tex = TextureUV{u1: X1+14.0*WIDTH, u2: X1+15.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '(' => tex = TextureUV{u1: X1+15.0*WIDTH, u2: X1+16.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        ')' => tex = TextureUV{u1: X1+16.0*WIDTH, u2: X1+17.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        ';' => tex = TextureUV{u1: X1+17.0*WIDTH, u2: X1+18.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '■' => tex = TextureUV{u1: X1+18.0*WIDTH+1.0, u2: X1+19.0*WIDTH-1.0, v1: 88.0, v2: 98.0},
        '[' => tex = TextureUV{u1: X1+19.0*WIDTH, u2: X1+20.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        ']' => tex = TextureUV{u1: X1+20.0*WIDTH, u2: X1+21.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
        '+' => tex = TextureUV{u1: X1+13.0*WIDTH, u2: X1+14.0*WIDTH, v1: Y6, v2: Y6+HEIGHT},
        '&' => tex = TextureUV{u1: X1+14.0*WIDTH, u2: X1+15.0*WIDTH, v1: Y6, v2: Y6+HEIGHT},
        '_' => tex = TextureUV{u1: X1+15.0*WIDTH, u2: X1+16.0*WIDTH, v1: Y6, v2: Y6+HEIGHT},
        '-' => tex = TextureUV{u1: X1+16.0*WIDTH, u2: X1+17.0*WIDTH, v1: Y6, v2: Y6+HEIGHT},
        '—' => tex = TextureUV{u1: X1+16.0*WIDTH, u2: X1+17.0*WIDTH, v1: Y6, v2: Y6+HEIGHT},
        '−' => tex = TextureUV{u1: X1+16.0*WIDTH, u2: X1+17.0*WIDTH, v1: Y6, v2: Y6+HEIGHT},
        '–' => tex = TextureUV{u1: X1+16.0*WIDTH, u2: X1+17.0*WIDTH, v1: Y6, v2: Y6+HEIGHT},
        '>' => tex = TextureUV{u1: X1+17.0*WIDTH, u2: X1+18.0*WIDTH, v1: Y6, v2: Y6+HEIGHT},
        '<' => tex = TextureUV{u1: X1+18.0*WIDTH, u2: X1+19.0*WIDTH, v1: Y6, v2: Y6+HEIGHT},
        _ => tex = TextureUV{u1: X1+10.0*WIDTH, u2: X1+11.0*WIDTH, v1: Y5, v2: Y5+HEIGHT},
    }

    tex
}

pub fn string_to_uv(text: &str) -> Vec<TextureUV> {

    let mut coords = Vec::new();

    let letters: Vec<char> = text.chars().collect();

    for letter in letters {
        coords.push(char_to_uv(letter))
    }

    for c in 0..coords.len() {
        coords[c].normalize(TEXWIDTH, TEXHEIGHT);
    }

    coords
}

fn vec_max(vect: &Vec<f32>) -> f32{
    let mut m = 0.0;
    for e in 0..vect.len() {
        if m < vect[e] {m = vect[e]}
    }
    m
}
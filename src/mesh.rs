use crate::text;
use crate::settings;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// #[repr(C)]
// struct Vec4 {
//     x: f32,
//     y: f32,
//     z: f32,
//     w: f32,
// }

#[derive(Debug, Clone, Copy)]
pub struct TextureUV {
    pub u1: f32,
    pub u2: f32,
    pub v1: f32,
    pub v2: f32,
}

impl TextureUV {
    pub fn normalize(&mut self, width: f32, height: f32) {
        self.u1 = self.u1 / width;
        self.u2 = self.u2 / width;
        self.v1 = self.v1 / height;
        self.v2 = self.v2 / height;
    }
}

#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    uv: Vec2,
    act: f32,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i16>,
    pub num: i32,
}

impl Mesh {
    pub fn new_overlay(overlay: &text::Overlay, scalex: f32, scaley: f32) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();
        let mut idx = 0;

        let mut tex_uv;

        for s in 0..overlay.lines.len() {
            let coords = text::string_to_uv(&overlay.lines[s]);

            for l in 0..coords.len() {
                tex_uv = coords[l];

                let lf = l as f32;

                let x = (overlay.line_x[s] + (lf + 1.0) * text::WIDTH * overlay.scale) * scalex;
                let y = overlay.line_y[s] * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    },
                    act: 0.0,
                }); // top right
                let x = (overlay.line_x[s] + (lf + 1.0) * text::WIDTH * overlay.scale) * scalex;
                let y = (overlay.line_y[s] + overlay.line_height) * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    },
                    act: 0.0,
                }); // bottom right
                let x = (overlay.line_x[s] + lf * text::WIDTH * overlay.scale) * scalex;
                let y = (overlay.line_y[s] + overlay.line_height) * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    },
                    act: 0.0,
                }); // bottom left
                let x = (overlay.line_x[s] + lf * text::WIDTH * overlay.scale) * scalex;
                let y = (overlay.line_y[s]) * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    },
                    act: 0.0,
                }); // top left

                indices.push(4 * idx + 0);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 2);
                indices.push(4 * idx + 2);
                indices.push(4 * idx + 3);
                indices.push(4 * idx + 0);

                idx = idx + 1;
            }
        }

        Mesh {
            vertices,
            indices,
            num: idx as i32,
        }
    }

    pub fn new_gui(gui: &text::GUI, scalex: f32, scaley: f32) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();
        let mut idx = 0;

        let mut tex_uv = text::string_to_uv("=")[0];

        let x = 0.5 * (1.0 + 1.2 * gui.max_width * scalex);
        let y = (gui.line_y[0] - 1.0 * gui.line_height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v1,
            },
            act: 0.0,
        }); // top right
        let x = 0.5 * (1.0 + 1.2 * gui.max_width * scalex);
        let y = (gui.line_y[gui.lines.len() - 1] + 2.0 * gui.line_height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom right
        let x = 0.5 * (1.0 - 1.2 * gui.max_width * scalex);
        let y = (gui.line_y[gui.lines.len() - 1] + 2.0 * gui.line_height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom left
        let x = 0.5 * (1.0 - 1.2 * gui.max_width * scalex);
        let y = (gui.line_y[0] - 1.0 * gui.line_height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v1,
            },
            act: 0.0,
        }); // top left

        indices.push(4 * idx + 0);
        indices.push(4 * idx + 1);
        indices.push(4 * idx + 2);
        indices.push(4 * idx + 2);
        indices.push(4 * idx + 3);
        indices.push(4 * idx + 0);

        idx = idx + 1;

        for s in 0..gui.lines.len() {
            let coords = text::string_to_uv(&gui.lines[s]);

            for l in 0..coords.len() {
                tex_uv = coords[l];

                let lf = l as f32;

                let x = (gui.line_x[s] + (lf + 1.0) * text::WIDTH * gui.scale) * scalex;
                let y = gui.line_y[s] * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v1,
                    },
                    act: gui.line_active[s] as f32,
                }); // top right
                let x = (gui.line_x[s] + (lf + 1.0) * text::WIDTH * gui.scale) * scalex;
                let y = (gui.line_y[s] + gui.line_height) * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u2,
                        y: tex_uv.v2,
                    },
                    act: gui.line_active[s] as f32,
                }); // bottom right
                let x = (gui.line_x[s] + lf * text::WIDTH * gui.scale) * scalex;
                let y = (gui.line_y[s] + gui.line_height) * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v2,
                    },
                    act: gui.line_active[s] as f32,
                }); // bottom left
                let x = (gui.line_x[s] + lf * text::WIDTH * gui.scale) * scalex;
                let y = (gui.line_y[s]) * scaley;
                vertices.push(Vertex {
                    pos: Vec3 { x, y, z: 0.0 },
                    uv: Vec2 {
                        x: tex_uv.u1,
                        y: tex_uv.v1,
                    },
                    act: gui.line_active[s] as f32,
                }); // top left

                indices.push(4 * idx + 0);
                indices.push(4 * idx + 1);
                indices.push(4 * idx + 2);
                indices.push(4 * idx + 2);
                indices.push(4 * idx + 3);
                indices.push(4 * idx + 0);

                idx = idx + 1;
            }
        }

        Mesh {
            vertices,
            indices,
            num: idx as i32,
        }
    }

    pub fn new_visuals(stats: &[i16; settings::SAMPLES]) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();
        let mut idx = 0;

        let mut tex_uv;

        let smax_f = 32767 as f32;

        let mut snorm: [f32; settings::SAMPLES] = [0.0; settings::SAMPLES];

        for l in 0..settings::SAMPLES {
            snorm[l] = stats[l].abs() as f32 / smax_f
        }

        let mut snorm2: [f32; settings::BINS] = [0.0; settings::BINS];

        

        for l in 0..settings::BINS {
            for k in 0..settings::BIN_SAMPLES {
                snorm2[l] += snorm[settings::BIN_SAMPLES*l+k] / settings::BIN_SAMPLES as f32;
            }
        }

        let x0: f32 = 0.1/settings::BINS as f32;

        let dx: f32 = 0.8/settings::BINS as f32;

        for l in 0..settings::BINS {
            let lf = l as f32 / settings::BINS as f32;

            tex_uv = TextureUV {
                u1: lf,
                u2: lf,
                v1: snorm2[l],
                v2: snorm2[l],
            };

            let x = lf+dx-0.5 + x0;
            let y = snorm2[l] - 0.5;
            vertices.push(Vertex {
                pos: Vec3 { x, y, z: 0.0 },
                uv: Vec2 {
                    x: tex_uv.u2,
                    y: tex_uv.v1,
                },
                act: 0.0,
            }); // top right
            let x = lf+dx-0.5 + x0;
            let y = -0.5;
            vertices.push(Vertex {
                pos: Vec3 { x, y, z: 0.0 },
                uv: Vec2 {
                    x: tex_uv.u2,
                    y: tex_uv.v2,
                },
                act: 0.0,
            }); // bottom right
            let x = lf-0.5 + x0;
            let y = -0.5;
            vertices.push(Vertex {
                pos: Vec3 { x, y, z: 0.0 },
                uv: Vec2 {
                    x: tex_uv.u1,
                    y: tex_uv.v2,
                },
                act: 0.0,
            }); // bottom left
            let x = lf-0.5 + x0;
            let y = snorm2[l] - 0.5;
            vertices.push(Vertex {
                pos: Vec3 { x, y, z: 0.0 },
                uv: Vec2 {
                    x: tex_uv.u1,
                    y: tex_uv.v1,
                },
                act: 0.0,
            }); // top left

            indices.push(4 * idx + 0);
            indices.push(4 * idx + 1);
            indices.push(4 * idx + 2);
            indices.push(4 * idx + 2);
            indices.push(4 * idx + 3);
            indices.push(4 * idx + 0);

            idx = idx + 1;
        }

        Mesh {
            vertices,
            indices,
            num: idx as i32,
        }
    }

    pub fn new_screen(asp: f32) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();

        let tex_uv = TextureUV {
            u1: 0.0,
            u2: 1.0,
            v1: 1.0,
            v2: 0.0,
        };

        let x = 1.0/asp;
        let y = 1.0;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v1,
            },
            act: 0.0,
        }); // top right
        let x = 1.0/asp;
        let y = -1.0;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom right
        let x = -1.0/asp;
        let y = -1.0;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom left
        let x = -1.0/asp;
        let y = 1.0;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v1,
            },
            act: 0.0,
        }); // top left

        indices.push(0);
        indices.push(1);
        indices.push(2);
        indices.push(2);
        indices.push(3);
        indices.push(0);

        Mesh {
            vertices,
            indices,
            num: 1,
        }
    }
}

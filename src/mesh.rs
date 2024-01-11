use crate::text;

#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
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
    pos: Vec3,
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

        let x = 0.5 * (1.0 + 1.5 * gui.max_width * scalex);
        let y = (gui.line_y[0] - 1.0 * gui.line_height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v1,
            },
            act: 0.0,
        }); // top right
        let x = 0.5 * (1.0 + 1.5 * gui.max_width * scalex);
        let y = (gui.line_y[gui.lines.len() - 1] + 2.0 * gui.line_height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom right
        let x = 0.5 * (1.0 - 1.5 * gui.max_width * scalex);
        let y = (gui.line_y[gui.lines.len() - 1] + 2.0 * gui.line_height) * scaley;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom left
        let x = 0.5 * (1.0 - 1.5 * gui.max_width * scalex);
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

    pub fn new_screen(
    ) -> Mesh {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<i16> = Vec::new();

        let tex_uv = TextureUV {
            u1: 0.0,
            u2: 1.0,
            v1: 1.0,
            v2: 0.0,
        };

        let x = 1.0;
        let y = 1.0;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v1,
            },
            act: 0.0,
        }); // top right
        let x = 1.0;
        let y = -1.0;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u2,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom right
        let x = -1.0;
        let y = -1.0;
        vertices.push(Vertex {
            pos: Vec3 { x, y, z: 0.0 },
            uv: Vec2 {
                x: tex_uv.u1,
                y: tex_uv.v2,
            },
            act: 0.0,
        }); // bottom left
        let x = -1.0;
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

use image::{self, EncodableLayout, ImageBuffer, Rgba};
use std::sync::{Arc, Mutex};
use miniquad::*;

use crate::assets;
use crate::mesh;
use crate::settings;
use crate::shaders;
use crate::text;
use crate::input::{TimeState, InputState};

pub struct Stage {
    ctx: Box<dyn RenderingBackend>,

    settings: settings::Settings,
    overlay: text::Overlay,
    gui: text::GUI,
    mesh: Vec<mesh::Mesh>,
    render_pass: RenderPass,
    pipeline: Vec<Pipeline>,
    bindings: Vec<Bindings>,
    state: Arc<Mutex<crate::State>>,

    time_state: TimeState,
    input_state: InputState,
}

impl Stage {
    pub fn new(state: Arc<Mutex<crate::State>>) -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        let settings = settings::Settings::init();
        let ass = assets::Ass::load();

        let overlay = text::Overlay::new_from(vec!["Text default"]);
        let gui = text::GUI::new_from(vec!["Text default"], settings.screen_width_f, settings.screen_height_f);

        let mesh_overlay = mesh::Mesh::new_overlay(
            &overlay,
            1.0 / settings.screen_width_f,
            1.0 / settings.screen_height_f,
        );
        let mesh_gui = mesh::Mesh::new_gui(
            &gui,
            1.0 / settings.screen_width_f,
            1.0 / settings.screen_height_f,
        );
        let mesh_screen = mesh::Mesh::new_screen();

        let vertex_buffer_overlay = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<mesh::Vertex>(settings::MAX_VERTICES_OVERLAY),
        );

        let vertex_buffer_gui = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<mesh::Vertex>(settings::MAX_VERTICES_GUI),
        );

        let vertex_buffer_screen = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&mesh_screen.vertices),
        );

        let index_buffer_overlay = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<i16>(2*settings::MAX_INDICES_OVERLAY),
        );

        let index_buffer_gui = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Stream,
            BufferSource::empty::<i16>(2*settings::MAX_INDICES_GUI),
        );

        let index_buffer_screen = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Stream,
            BufferSource::slice(&mesh_screen.indices),
        );

        let pixels: ImageBuffer<Rgba<u8>, Vec<u8>> = ass.font;
        let dims = pixels.dimensions();

        let mut t_params = TextureParams {
            kind: TextureKind::Texture2D,
            format: TextureFormat::RGBA8,
            wrap: TextureWrap::Clamp,
            min_filter: FilterMode::Nearest,
            mag_filter: FilterMode::Nearest,
            mipmap_filter: MipmapFilterMode::None,
            width: dims.0,
            height: dims.1,
            allocate_mipmaps: false,
        };

        let texture_overlay = ctx.new_texture_from_data_and_format(pixels.as_bytes(), t_params);

        t_params = TextureParams {
            kind: TextureKind::Texture2D,
            format: TextureFormat::RGBA8,
            wrap: TextureWrap::Clamp,
            min_filter: FilterMode::Linear,
            mag_filter: FilterMode::Linear,
            mipmap_filter: MipmapFilterMode::None,
            width: settings::WIDTH,
            height: settings::HEIGHT,
            allocate_mipmaps: false,
        };

        let texture = ctx.new_render_texture(t_params);

        let bindings_overlay = Bindings {
            vertex_buffers: vec![vertex_buffer_overlay],
            index_buffer: index_buffer_overlay,
            images: vec![texture_overlay],
        };

        let bindings_gui = Bindings {
            vertex_buffers: vec![vertex_buffer_gui],
            index_buffer: index_buffer_gui,
            images: vec![texture_overlay],
        };

        let bindings_screen = Bindings {
            vertex_buffers: vec![vertex_buffer_screen],
            index_buffer: index_buffer_screen,
            images: vec![texture],
        };

        let shader_overlay = ctx
            .new_shader(
                miniquad::ShaderSource::Glsl {
                    vertex: shaders::VERTEX_OVERLAY,
                    fragment: shaders::FRAGMENT_OVERLAY,
                },
                shaders::meta_overlay(),
            )
            .unwrap();

        let shader_gui = ctx
            .new_shader(
                miniquad::ShaderSource::Glsl {
                    vertex: shaders::VERTEX_GUI,
                    fragment: shaders::FRAGMENT_GUI,
                },
                shaders::meta_gui(),
            )
            .unwrap();

        let shader_screen = ctx
            .new_shader(
                miniquad::ShaderSource::Glsl {
                    vertex: shaders::VERTEX_SCREEN,
                    fragment: shaders::FRAGMENT_SCREEN,
                },
                shaders::meta_screen(),
            )
            .unwrap();

        let mut p_params = PipelineParams {
            cull_face: CullFace::Nothing,
            front_face_order: FrontFaceOrder::CounterClockwise,
            depth_test: Comparison::Always,
            depth_write: false,      
            depth_write_offset: None,
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
            ),
            alpha_blend: Some(BlendState::new(Equation::Add, 
                BlendFactor::Value(BlendValue::SourceAlpha), 
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha))),
            stencil_test: None,
            color_write: (true, true, true, true),
            primitive_type: PrimitiveType::Triangles,
        };

        let pipeline_overlay = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
                VertexAttribute::new("act", VertexFormat::Float1),
            ],
            shader_overlay,
            p_params,
        );

        let pipeline_gui = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
                VertexAttribute::new("act", VertexFormat::Float1),
            ],
            shader_gui,
            p_params,
        );

        p_params = PipelineParams {
            cull_face: CullFace::Nothing,
            front_face_order: FrontFaceOrder::CounterClockwise,
            depth_test: Comparison::Always,
            depth_write: false,      
            depth_write_offset: None,
            color_blend: None,
            alpha_blend: None,
            stencil_test: None,
            color_write: (true, true, true, true),
            primitive_type: PrimitiveType::Triangles,
        };

        let pipeline_screen = ctx.new_pipeline_with_params(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
                VertexAttribute::new("act", VertexFormat::Float1),
            ],
            shader_screen,
            p_params,
        );

        let gui = text::GUI::new_from(vec!["Text default"], settings.screen_width_f, settings.screen_height_f);

        let render_pass = ctx.new_render_pass(texture, None);

        Stage {
            ctx,

            settings,
            overlay: text::Overlay::new_from(vec!["Text default"]),
            gui,
            pipeline: vec![pipeline_overlay, pipeline_gui, pipeline_screen],
            bindings: vec![bindings_overlay, bindings_gui, bindings_screen],
            mesh: vec![mesh_overlay, mesh_gui, mesh_screen],
            render_pass,
            state,

            time_state: TimeState::init(),
            input_state: InputState::init(),
        }
    }

    fn show_gui(&mut self) {
        let s_display = self.state.lock().unwrap();
        self.overlay = text::Overlay::new_from(vec![
            //&format!("FPS: {}.", self.time_state.fps + 1),
            &format!("[Space] to pause or continue."),
            &format!("[S] to skip, [Esc] to exit."),
            &s_display.message,
        ]);
        self.gui = text::GUI::new_from(vec![
            &format!("Current directory"),
            &s_display.dir_name,
            &format!("Now playing track <{}>", s_display.file_num),
            &s_display.file_name,
        ], self.settings.screen_width_f, self.settings.screen_height_f);
        drop(s_display);
        self.gui.line_active[1] = 1;
        self.gui.line_active[3] = 1;
    }
}

impl EventHandler for Stage {

    // ============================
    // UPDATE
    // ============================

    fn update(&mut self) {
        self.time_state.frame_time(&mut self.settings);

        self.show_gui();

        if self.input_state.keys.esc {
            miniquad::window::quit()
        }

        if self.input_state.keys.s && self.input_state.apply_change {
            let mut s_main = self.state.lock().unwrap();
            if s_main.play {
                s_main.skip = true;
                s_main.message = format!("*** track skipped")
            } else {
                s_main.message = format!("*** unpause to skip")
            }
            drop(s_main);
            self.input_state.apply_change = false;
        }

        if self.input_state.keys.space && self.input_state.apply_change {
            let mut s_main = self.state.lock().unwrap();
            s_main.play = !s_main.play;
            s_main.message = format!("*** paused: {}", !s_main.play);
            drop(s_main);
            self.input_state.apply_change = false;
        }
        
        self.mesh[0] = mesh::Mesh::new_overlay(
            &self.overlay,
            1.0 / self.settings.screen_width_f,
            1.0 / self.settings.screen_height_f,
        );
        self.mesh[1] = mesh::Mesh::new_gui(
            &self.gui,
            1.0 / self.settings.screen_width_f,
            1.0 / self.settings.screen_height_f,
        );
    }

    // ============================
    // DRAW
    // ============================

    fn draw(&mut self) {
        window::show_mouse(self.gui.show);

        self.ctx.begin_default_pass(PassAction::default());

        self.ctx.apply_pipeline(&self.pipeline[2]);

        self.ctx.apply_bindings(&self.bindings[2]);

        self.ctx
            .apply_uniforms(miniquad::UniformsSource::table(&shaders::UniformsScreen {
            }));

        self.ctx.draw(0, self.mesh[2].num * 6, 1);

        self.ctx.end_render_pass();

        self.ctx
            .begin_pass(Some(self.render_pass), PassAction::clear_color(settings::CLR4.0, settings::CLR4.1, settings::CLR4.2, 1.0));

        for j in 0..2 {
            self.ctx.buffer_update(self.bindings[j].vertex_buffers[0], BufferSource::slice(&self.mesh[j].vertices));
            self.ctx.buffer_update(self.bindings[j].index_buffer, BufferSource::slice(&self.mesh[j].indices));
        }

        self.ctx.apply_pipeline(&self.pipeline[0]);

        self.ctx.apply_bindings(&self.bindings[0]);

        self.ctx
            .apply_uniforms(miniquad::UniformsSource::table(&shaders::UniformsOverlay {
                fontcolor: self.overlay.font_col,
            }));

        self.ctx.draw(0, self.mesh[0].num * 6, 1);

        if self.gui.show {
            self.ctx.apply_pipeline(&self.pipeline[1]);

            self.ctx.apply_bindings(&self.bindings[1]);
    
            self.ctx
                .apply_uniforms(miniquad::UniformsSource::table(&shaders::UniformsGUI {
                    fontcolor: self.gui.font_col,
                    actcolor: self.gui.act_col,
                }));
    
            self.ctx.draw(0, self.mesh[1].num * 6, 1);    
        }
        
        self.ctx.end_render_pass();

        self.ctx.commit_frame();

        self.time_state.last_frame = date::now();
    }

    // ============================
    // INPUT HANDLING
    // ============================

    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.input_state.keys.read_key(keycode, true);
        if !self.input_state.apply_change {
            self.input_state.apply_change = true
        }
    }

    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        self.input_state.keys.read_key(keycode, false);
        self.input_state.apply_change = false
    }

    fn resize_event(&mut self, width: f32, height: f32) {
        self.settings.screen_change(width, height);
    }
}
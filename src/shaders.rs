use miniquad::*;

pub const VERTEX_OVERLAY: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;
in float act;

out vec2 texcoord;

void main() {
    gl_Position = vec4((pos.x-0.5)*2.0, (0.5-pos.y)*2.0, 0.0, 1.0);
    texcoord = uv;
}"#;

pub const FRAGMENT_OVERLAY: &str = r#"#version 330 core
in vec2 texcoord;

out vec4 FragColor;

uniform sampler2D tex;
uniform vec4 fontcolor;

vec4 col;

void main() {
    col = texture(tex, texcoord);

    FragColor = vec4(fontcolor.xyz,col.w);
}"#;

pub const VERTEX_GUI: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;
in float act;

uniform vec4 fontcolor;
uniform vec4 actcolor;

out vec2 texcoord;
out vec4 cols;
out vec2 posxy;

void main() {
    posxy = vec2((pos.x-0.5)*2.0, (0.5-pos.y)*2.0);
    gl_Position = vec4(posxy + act*vec2(0.005,-0.005), 0.0, 1.0);
    texcoord = uv;
    cols = actcolor*act + fontcolor*(1.0-act);
}"#;

pub const FRAGMENT_GUI: &str = r#"#version 330 core
in vec2 texcoord;
in vec4 cols;
in vec2 posxy;

out vec4 FragColor;

uniform sampler2D tex;

vec4 col;

void main() {
    col = texture(tex, texcoord);
    FragColor = vec4(col.xyz+(1-col.xyz)*cols.xyz,col.w);
    
}"#;

pub const VERTEX_VISUALS: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;
in float act;

out vec2 texcoord;

void main() {
    texcoord = vec2(-pos.y, uv.y);
    gl_Position = vec4(pos, 1.0);
}"#;

pub const FRAGMENT_VISUALS: &str = r#"#version 330 core
in vec2 texcoord;

out vec4 FragColor;

uniform vec4 fontcolor;

void main() {
    FragColor = fontcolor*vec4(texcoord.x, 0.5*texcoord.x, texcoord.y, 1.0);
}"#;

pub const VERTEX_SCREEN: &str = r#"#version 330 core
in vec3 pos;
in vec2 uv;
in float act;

out vec2 texcoord;

void main() {
    gl_Position = vec4(pos, 1.0);
    texcoord = uv;
}"#;

pub const FRAGMENT_SCREEN: &str = r#"#version 330 core
in vec2 texcoord;

out vec4 FragColor;

uniform sampler2D tex;

void main() {
    FragColor = texture(tex, texcoord);
}"#;

pub fn meta_overlay() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("fontcolor", UniformType::Float4),
            ],
        },
    }
}

pub fn meta_visuals() -> ShaderMeta {
    ShaderMeta {
        images: vec![],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("fontcolor", UniformType::Float4),
            ],
        },
    }
}

pub fn meta_gui() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("fontcolor", UniformType::Float4),
                UniformDesc::new("actcolor", UniformType::Float4),
            ],
        },
    }
}

pub fn meta_screen() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![
            ],
        },
    }
}

#[repr(C)]
pub struct UniformsOverlay {
    pub fontcolor: (f32, f32, f32, f32),
}

#[repr(C)]
pub struct UniformsGUI {
    pub fontcolor: (f32, f32, f32, f32),
    pub actcolor: (f32, f32, f32, f32),
}

#[repr(C)]
pub struct UniformsVisuals {
    pub fontcolor: (f32, f32, f32, f32),
}

#[repr(C)]
pub struct UniformsScreen {
}

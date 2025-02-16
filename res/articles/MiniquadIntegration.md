# Miniquad/Macroquad Integration
We will first start with Miniquad integration, for Macroquad integration skip
to the next section.
To begin with install spf, miniquad, and macroquad to your project via:
```sh
cargo add spf
```

## Only-Miniquad Integeation
In this section we will integrate spf.rs with miniquad.rs by building a simple app that renders onto the screen the current letter pressed on the keyboard. We will only explain the rendering code with little depth, as the focus is on integration. You may want to learn OpenGL and Miniquad graphics API on your own to better understand the sample.

### Setup
Lets first add miniquad to our Rust project with the following command:
```sh
cargo add miniquad
```
Great, we can now bring some modules we will need into scope:
```rs
use miniquad::*;
use spf::*;
// Required trait to read file contents into Vec<u8> buffer.
use std::io::Read;
```
Now we setup some structs for rendering:
```rs
#[repr(C)]
struct Vec2 {
    x: f32,
    y: f32,
}
#[repr(C)]
struct Vertex {
    pos: Vec2,
    uv: Vec2,
}

```
We can define our OpenGL shaders for rendering a texture:
```rs
mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 in_pos;
    attribute vec2 in_uv;

    varying lowp vec2 texcoord;

    void main() {
        gl_Position = vec4(in_pos, 0, 1);
        texcoord = in_uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;

    uniform sampler2D tex;

    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout { uniforms: vec![] },
        }
    }
}
```
We can now begin writing our rendering engine:
```rs
struct Stage {
    ctx: Box<dyn RenderingBackend>,

    pipeline: Pipeline,
    bindings: Bindings,
}
```
```rs
impl Stage {
    pub fn new() -> Stage {
        let mut ctx: Box<dyn RenderingBackend> = window::new_rendering_backend();

        #[rustfmt::skip]
        let vertices: [Vertex; 4] = [
            Vertex { pos : Vec2 { x: -1.0, y: -1.0 }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y: -1.0 }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  1.0, y:  1.0 }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -1.0, y:  1.0 }, uv: Vec2 { x: 0., y: 1. } },
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .open("./fg_miniscript.spf")
            .unwrap();
        let mut buffer: Vec<u8> = vec![];
        file.read_to_end(&mut buffer).unwrap();
        let font: spf::core::SimplePixelFont =
            spf::core::SimplePixelFont::unchecked_from_vec_u8(buffer);
        let printer = spf::printer::Printer {
            font: font,
            letter_spacing: 1,
        };
        const letter: char = 'w';
        println!("{:?}", printer.font.size);
        let surface = printer.new_text(letter.to_string());
        let pixels =
            surface.flatten_replace(&[vec![0u8, 0u8, 0u8, 0u8], vec![255u8, 0u8, 0u8, 255u8]]);

        println!("{:?}", pixels);

        let texture =
            ctx.new_texture_from_rgba8(surface.width as u16, surface.height as u16, &pixels);
        ctx.texture_set_filter(texture, FilterMode::Nearest, MipmapFilterMode::None);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![texture],
        };

        let shader = ctx
            .new_shader(
                match ctx.info().backend {
                    Backend::OpenGl => Some(ShaderSource::Glsl {
                        vertex: shader::VERTEX,
                        fragment: shader::FRAGMENT,
                    }),
                    _ => None,
                }
                .unwrap(),
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
                VertexAttribute::new("in_uv", VertexFormat::Float2),
            ],
            shader,
            PipelineParams::default(),
        );

        Stage {
            pipeline,
            bindings,
            ctx,
        }
    }
}

impl EventHandler for Stage {
    fn update(&mut self) {}

    fn draw(&mut self) {
        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);

        self.ctx.draw(0, 6, 1);

        self.ctx.end_render_pass();

        self.ctx.commit_frame();
    }
}

fn main() {
    let conf = conf::Conf::default();
    miniquad::start(conf, move || Box::new(Stage::new()));
}

```
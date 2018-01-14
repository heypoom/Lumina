extern crate glium;
extern crate image;

use self::glium::{index, Display, IndexBuffer, Program, Surface, VertexBuffer};
use self::glium::glutin::*;
use self::glium::backend::Facade;
use self::glium::texture::{RawImage2d, SrgbTexture2d, Texture2d};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Copy, Clone)]
pub struct Vertex {
  pos: [f32; 3],
  normal: [f32; 3],
  uv: [f32; 2],
}

implement_vertex!(Vertex, pos, normal, uv);

fn get_shader<T: Facade>(name: &str, display: &T) -> Program {
  let mut vert_src = String::new();
  let mut frag_src = String::new();

  let vert_path = format!("./shaders/{}.vert", name);
  let frag_path = format!("./shaders/{}.frag", name);

  let mut vert_file = File::open(vert_path).unwrap();
  let mut frag_file = File::open(frag_path).unwrap();

  vert_file.read_to_string(&mut vert_src).unwrap();
  frag_file.read_to_string(&mut frag_src).unwrap();

  Program::from_source(display, &vert_src, &frag_src, None).unwrap()
}

fn raw_texture(name: &str) -> RawImage2d<u8> {
  let image_path = format!("./textures/{}.png", name);
  let image_file = File::open(image_path).unwrap();
  let image_reader = BufReader::new(image_file);

  let image = image::load(image_reader, image::PNG).unwrap().to_rgba();
  let dimensions = image.dimensions();
  let raw: &[u8] = &image.into_raw();

  RawImage2d::from_raw_rgba_reversed(raw, dimensions)
}

fn view_matrix(pos: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
  let f = {
    let f = direction;
    let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
    let len = len.sqrt();

    [f[0] / len, f[1] / len, f[2] / len]
  };

  let s = [
    up[1] * f[2] - up[2] * f[1],
    up[2] * f[0] - up[0] * f[2],
    up[0] * f[1] - up[1] * f[0],
  ];

  let s_norm = {
    let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
    let len = len.sqrt();

    [s[0] / len, s[1] / len, s[2] / len]
  };

  let u = [
    f[1] * s_norm[2] - f[2] * s_norm[1],
    f[2] * s_norm[0] - f[0] * s_norm[2],
    f[0] * s_norm[1] - f[1] * s_norm[0],
  ];

  let p = [
    -pos[0] * s_norm[0] - pos[1] * s_norm[1] - pos[2] * s_norm[2],
    -pos[0] * u[0] - pos[1] * u[1] - pos[2] * u[2],
    -pos[0] * f[0] - pos[1] * f[1] - pos[2] * f[2],
  ];

  [
    [s_norm[0], u[0], f[0], 0.0],
    [s_norm[1], u[1], f[1], 0.0],
    [s_norm[2], u[2], f[2], 0.0],
    [p[0], p[1], p[2], 1.0],
  ]
}

fn perspective(dimensions: (u32, u32)) -> [[f32; 4]; 4] {
  let (width, height) = dimensions;
  let aspect_ratio = height as f32 / width as f32;

  const PI: f32 = 3.141592;
  let fov: f32 = PI / 3.0;
  let zfar = 1024.0;
  let znear = 0.1;

  let f = 1.0 / (fov / 2.0).tan();

  [
    [f * aspect_ratio, 0.0, 0.0, 0.0],
    [0.0, f, 0.0, 0.0],
    [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
    [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
  ]
}

fn load_texture<T: Facade>(name: &str, display: &T) -> (SrgbTexture2d, Texture2d, Texture2d) {
  let normal_name = format!("{}_n", name);
  let specular_name = format!("{}_s", name);

  let diffuse = SrgbTexture2d::new(display, raw_texture(name)).unwrap();
  let normal = Texture2d::new(display, raw_texture(&normal_name)).unwrap();
  let specular = Texture2d::new(display, raw_texture(&specular_name)).unwrap();

  (diffuse, normal, specular)
}

fn handle_events(display: Display, events_loop: &mut EventsLoop) {
  let mut closed = false;

  #[cfg_attr(rustfmt, rustfmt_skip)]
  let shape = VertexBuffer::new(&display, &[
    // Front
    Vertex { pos: [-1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [1.0, 0.0] },
    Vertex { pos: [ 1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [0.0, 0.0] },
    Vertex { pos: [ 1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [0.0, 1.0] },
    Vertex { pos: [-1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [1.0, 1.0] },

    // Right
    Vertex { pos: [ 1.0, -1.0,  1.0], normal: [ 1.0,  0.0,  0.0], uv: [1.0, 1.0] },
    Vertex { pos: [ 1.0, -1.0, -1.0], normal: [ 1.0,  0.0,  0.0], uv: [1.0, 0.0] },
    Vertex { pos: [ 1.0,  1.0, -1.0], normal: [ 1.0,  0.0,  0.0], uv: [0.0, 0.0] },
    Vertex { pos: [ 1.0,  1.0,  1.0], normal: [ 1.0,  0.0,  0.0], uv: [0.0, 1.0] },

    // Back
    Vertex { pos: [-1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [0.0, 0.0] },
    Vertex { pos: [-1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [0.0, 1.0] },
    Vertex { pos: [ 1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [1.0, 1.0] },
    Vertex { pos: [ 1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [1.0, 0.0] },

    // Left
    Vertex { pos: [-1.0, -1.0,  1.0], normal: [-1.0,  0.0,  0.0], uv: [0.0, 1.0] },
    Vertex { pos: [-1.0,  1.0,  1.0], normal: [-1.0,  0.0,  0.0], uv: [1.0, 1.0] },
    Vertex { pos: [-1.0,  1.0, -1.0], normal: [-1.0,  0.0,  0.0], uv: [1.0, 0.0] },
    Vertex { pos: [-1.0, -1.0, -1.0], normal: [-1.0,  0.0,  0.0], uv: [0.0, 0.0] },

    // Bottom
    Vertex { pos: [-1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0], uv: [0.0, 1.0] },
    Vertex { pos: [-1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0], uv: [0.0, 0.0] },
    Vertex { pos: [ 1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0], uv: [1.0, 0.0] },
    Vertex { pos: [ 1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0], uv: [1.0, 1.0] },

    // Top
    Vertex { pos: [-1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0], uv: [1.0, 1.0] },
    Vertex { pos: [ 1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0], uv: [0.0, 1.0] },
    Vertex { pos: [ 1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0], uv: [1.0, 0.0] },
    Vertex { pos: [-1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0], uv: [1.0, 0.0] },
  ]).unwrap();

  #[cfg_attr(rustfmt, rustfmt_skip)]
  let indices_list = &[
    // Front
    0u16, 2, 1, 0, 3, 2,

    // Right
    4, 6, 5, 4, 7, 6,

    // Back
    8, 10, 9, 8, 11, 10,

    // Left
    12, 14, 13, 12, 15, 14,

    // Bottom
    16, 18, 17, 16, 19, 18,

    // Top
    20, 22, 21, 20, 23, 22,
  ];

  let indices = IndexBuffer::new(&display, index::PrimitiveType::TrianglesList, indices_list).unwrap();

  let shader = get_shader("Basic", &display);

  let (diffuse, normal_map, specular_map) = load_texture("command_block", &display);

  let mut t: f32 = -0.5;

  let mut x: f32 = -0.5;
  let mut y: f32 = -0.5;
  let mut z: f32 = 2.0;

  while !closed {
    t += 0.0002;

    if t > 0.5 {
      t = -0.5;
    }

    let model = [
      [t.cos(), t.sin(), 0.0, 0.0],
      [-t.sin(), t.cos(), 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, z, 1.0f32],
    ];

    let cam_pos = [z, x, 1.0];
    let cam_facing = [x, 1.0, 1.0];
    let cam_up = [0.0, 1.0, 0.0];

    let view = view_matrix(&cam_pos, &cam_facing, &cam_up);

    let mut target = display.draw();

    let perspective = perspective(target.get_dimensions());

    // let light = [1.7, 0.3, 0.7f32];
    let light = [-1.0, 0.4, 0.9f32];

    let uniforms = uniform! {
      model: model,
      view: view,
      perspective: perspective,
      diffuse_tex: &diffuse,
      normal_tex: &normal_map,
      specular_tex: &specular_map,
      u_light: light
    };

    let params = glium::DrawParameters {
      depth: glium::Depth {
        test: glium::draw_parameters::DepthTest::IfLess,
        write: true,
        ..Default::default()
      },
      // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
      ..Default::default()
    };

    target.clear_color_and_depth((0.93, 0.93, 0.93, 1.0), 1.0);

    target.draw(&shape, &indices, &shader, &uniforms, &params).unwrap();

    target.finish().unwrap();

    events_loop.poll_events(|ev| match ev {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Closed => closed = true,
        WindowEvent::KeyboardInput { input, .. } => match input.scancode {
          13 => {
            z -= 0.1;
          }
          0 => {
            x -= 0.1;
          }
          1 => {
            z += 0.1;
          }
          2 => {
            x += 0.1;
          }
          _ => println!("Key: {}", input.scancode),
        },
        _ => (),
      },
      _ => (),
    });
  }
}

pub fn init() {
  let mut events_loop = EventsLoop::new();

  let window = WindowBuilder::new()
    .with_dimensions(1024, 768)
    .with_title("Lumina 0.1");

  let context = ContextBuilder::new().with_depth_buffer(24);

  let display = Display::new(window, context, &events_loop).unwrap();

  handle_events(display, &mut events_loop);
}

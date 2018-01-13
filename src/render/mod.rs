extern crate image;
extern crate glium;

use self::glium::{Display, Surface, VertexBuffer, IndexBuffer, Program, index};
use self::glium::glutin::*;
use self::glium::backend::Facade;
use self::glium::texture::{Texture2d, RawImage2d};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

mod teapot;

#[derive(Copy, Clone)]
pub struct Vertex {
  position: [f32; 2],
  tex_coords: [f32; 2]
}

implement_vertex!(Vertex, position, tex_coords);

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

fn get_texture<T: Facade>(name: &str, display: &T) -> Texture2d {
  let image_path = format!("./textures/{}.png", name);
  let image_file = File::open(image_path).unwrap();
  let image_reader = BufReader::new(image_file);

  let image = image::load(image_reader, image::PNG).unwrap().to_rgba();

  let image_dimensions = image.dimensions();
  let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

  Texture2d::new(display, image).unwrap()
}

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
  let f = {
    let f = direction;
    let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
    let len = len.sqrt();

    [f[0] / len, f[1] / len, f[2] / len]
  };

  let s = [
    up[1] * f[2] - up[2] * f[1],
    up[2] * f[0] - up[0] * f[2],
    up[0] * f[1] - up[1] * f[0]
  ];

  let s_norm = {
    let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
    let len = len.sqrt();

    [s[0] / len, s[1] / len, s[2] / len]
  };

  let u = [
    f[1] * s_norm[2] - f[2] * s_norm[1],
    f[2] * s_norm[0] - f[0] * s_norm[2],
    f[0] * s_norm[1] - f[1] * s_norm[0]
  ];

  let p = [
    -position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
    -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
    -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]
  ];

  [
    [s_norm[0], u[0], f[0], 0.0],
    [s_norm[1], u[1], f[1], 0.0],
    [s_norm[2], u[2], f[2], 0.0],
    [p[0], p[1], p[2], 1.0],
  ]
}

fn handle_events(display: Display, events_loop: &mut EventsLoop) {
  let mut closed = false;

  let positions = VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
  let normals = VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
  let indices = IndexBuffer::new(&display, index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

  let shader = get_shader("Basic", &display);
  let texture = get_texture("hello", &display);

  let mut t: f32 = -0.5;
  let mut x: f32 = -0.5;
  let mut z: f32 = 2.0;

  while !closed {
    t += 0.0002;

    if t > 0.5 {
        t = -0.5;
    }

    let model = [
      [0.01, 0.0, 0.0, 0.0],
      [0.0, 0.01, 0.0, 0.0],
      [0.0, 0.0, 0.01, 0.0],
      [0.0, 0.0, z, 1.0f32]
    ];

    let view = view_matrix(
      &[2.0, -1.0, 1.0],
      &[-2.0, 1.0, 1.0],
      &[0.0, 1.0, 0.0]
    );

    let mut target = display.draw();
    let vertices = (&positions, &normals);

    let perspective = {
      let (width, height) = target.get_dimensions();
      let aspect_ratio = height as f32 / width as f32;

      let PI: f32 = 3.141592;
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
    };

    let light = [1.4, 0.4, -0.7f32];

    let uniforms = uniform! {
      model: model,
      view: view,
      perspective: perspective,
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
    target.draw(vertices, &indices, &shader, &uniforms, &params).unwrap();
    target.finish().unwrap();

    events_loop.poll_events(|ev| {
      match ev {
        Event::WindowEvent { event, .. } => {
          match event {
            WindowEvent::Closed => closed = true,
            WindowEvent::KeyboardInput { input, .. } => {
              match input.scancode {
                13 => {
                  println!("W");
                  z -= 0.1;
                },
                0 => {
                  println!("A");
                  x -= 0.1;
                },
                1 => {
                  println!("S");
                  z += 0.1;
                },
                2 => {
                  println!("D");
                  x += 0.1;
                }
                _ => ()
              }
            }
            _ => (),
          }
        },
        _ => (),
      }
    });
  }
}

pub fn init() {
  let mut events_loop = EventsLoop::new();

  let window = WindowBuilder::new()
    .with_dimensions(1024, 768)
    .with_title("Lumina 0.1");

  let context = ContextBuilder::new()
    .with_depth_buffer(24);

  let display = Display::new(window, context, &events_loop).unwrap();

  handle_events(display, &mut events_loop);
}


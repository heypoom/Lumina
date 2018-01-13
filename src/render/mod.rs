extern crate image;
extern crate glium;

use self::glium::{Display, Surface, VertexBuffer, Program, index};
use self::glium::glutin::*;
use self::glium::backend::Facade;
use self::glium::texture::{Texture2d, RawImage2d};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Copy, Clone)]
pub struct Vertex {
  position: [f32; 2],
  tex_coords: [f32; 2]
}

impl Vertex {
  fn new(x: f32, y: f32, tx: f32, ty: f32) -> Vertex {
    Vertex { position: [x, y], tex_coords: [tx, ty] }
  }
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

fn handle_events(display: Display, events_loop: &mut EventsLoop) {
  let mut closed = false;

  let v1 = Vertex::new(-0.5, -0.5, 0.0, 0.0);
  let v2 = Vertex::new(0.0, 0.5, 0.0, 1.0);
  let v3 = Vertex::new(0.5, -0.15, 1.0, 0.0);

  let shape = vec![v1, v2, v3];
  let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
  let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

  let shader = get_shader("Basic", &display);
  let texture = get_texture("hello", &display);

  let mut t: f32 = -0.5;
  let mut x: f32 = -0.5;

  while !closed {
    t += 0.0002;

    if t > 0.5 {
        t = -0.5;
    }

    let uniforms = uniform! {
      matrix: [
        [ t.cos(), t.sin(), 0.0, 0.0],
        [-t.sin(), t.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.5, 0.0, 0.5, 1.0f32],
      ],
      player_x: x,
      tex: &texture
    };

    let mut target = display.draw();

    target.clear_color(0.93, 0.93, 0.93, 1.0);
    target.draw(&vertex_buffer, &indices, &shader, &uniforms, &Default::default()).unwrap();
    target.finish().unwrap();

    events_loop.poll_events(|ev| {
      match ev {
        Event::WindowEvent { event, .. } => {
          match event {
            WindowEvent::Closed => closed = true,
            WindowEvent::KeyboardInput { input, .. } => {
              match input.scancode {
                0 => {
                  println!("A");
                },
                1 => {
                  println!("S");
                  x -= 0.005;
                },
                2 => {
                  println!("D");
                },
                13 => {
                  println!("W");
                  x += 0.005;
                },
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

  let context = ContextBuilder::new();
  let display = Display::new(window, context, &events_loop).unwrap();

  handle_events(display, &mut events_loop);
}


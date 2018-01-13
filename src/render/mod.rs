extern crate glium;

use self::glium::{Display, Surface, VertexBuffer, Program, index};
use self::glium::glutin::*;

use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
pub struct Vertex {
  position: [f32; 2]
}

impl Vertex {
  fn new(x: f32, y: f32) -> Vertex {
    Vertex { position: [x, y] }
  }
}

implement_vertex!(Vertex, position);

fn get_shader_src<'a>(name: &str) -> (String, String) {
  let mut vert_src = String::new();
  let mut frag_src = String::new();

  let vert_path = format!("./shaders/{}.vert", name);
  let frag_path = format!("./shaders/{}.frag", name);

  let mut vert_file = File::open(vert_path).unwrap();
  let mut frag_file = File::open(frag_path).unwrap();

  vert_file.read_to_string(&mut vert_src).unwrap();
  frag_file.read_to_string(&mut frag_src).unwrap();

  (vert_src, frag_src)
}

fn handle_events(display: Display, events_loop: &mut EventsLoop) {
  let mut closed = false;

  let v1 = Vertex::new(-0.5, -0.5);
  let v2 = Vertex::new(0.0, 0.5);
  let v3 = Vertex::new(0.5, -0.15);

  let shape = vec![v1, v2, v3];
  let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
  let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

  let (vertex, fragment) = get_shader_src("Basic");
  let shader = Program::from_source(&display, &vertex, &fragment, None).unwrap();

  let mut t: f32 = -0.5;

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
        [0.0, 0.0, 0.0, 1.0f32],
      ]
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


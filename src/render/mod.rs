extern crate glium;

use self::glium::{Display, Surface, VertexBuffer, Program, index};
use self::glium::glutin::*;

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

fn handle_events(display: Display, events_loop: &mut EventsLoop) {
  let mut closed = false;

  while !closed {
    let v1 = Vertex::new(-0.5, -0.5);
    let v2 = Vertex::new(0.0, 0.5);
    let v3 = Vertex::new(0.5, -0.15);

    let shape = vec![v1, v2, v3];
    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
    let indices = index::NoIndices(index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(0.5, 0.5, 0.5, 1.0);
        }
    "#;

    let program = Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut target = display.draw();
    target.clear_color(0.93, 0.93, 0.93, 1.0);
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
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


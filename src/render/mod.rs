extern crate glium;
extern crate image;

use self::glium::{index, Display, IndexBuffer, Program, Surface, VertexBuffer};
use self::glium::glutin::*;
use self::glium::backend::Facade;
use self::glium::texture::{RawImage2d, SrgbTexture2d, Texture2d};

mod game;
mod cube;
mod shader;
mod texture;
mod matrix;
mod camera;

use self::camera::Camera;

pub const PI: f32 = 3.141592;

fn handle_events(display: Display, events_loop: &mut EventsLoop) {
  let mut closed = false;

  while !closed {
    events_loop.poll_events(|ev| match ev {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Closed => closed = true,
        WindowEvent::KeyboardInput { input, .. } => {
          let debug_str = format!("Lumina v0.1 | x {}, y {}, z {}, pitch {}, yaw {}", x, y, z, pitch, yaw);
          &display.gl_window().set_title(&debug_str);


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

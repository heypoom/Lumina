extern crate glium;

use self::glium::Display;
use self::glium::glutin::*;

fn handle_events(events_loop: &mut EventsLoop) {
  let mut closed = false;

  while !closed {
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

fn create_window() -> Display {
  let mut events_loop = EventsLoop::new();

  let window = WindowBuilder::new()
    .with_dimensions(1024, 768)
    .with_title("Lumina 0.1");

  let context = ContextBuilder::new();
  let display = Display::new(window, context, &events_loop).unwrap();

  handle_events(&mut events_loop);

  display
}

pub fn init() {
  let display = create_window();
}


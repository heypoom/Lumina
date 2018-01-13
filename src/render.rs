extern crate glium;

use self::glium::{Display, Surface};
use self::glium::glutin::*;

fn render(display: &Display) {
  let mut target = display.draw();
  target.clear_color(0.0, 0.0, 1.0, 1.0);
  target.finish().unwrap();
}

fn handle_events(display: &Display, events_loop: &mut EventsLoop) {
  let mut closed = false;

  while !closed {
    render(&display);

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

  handle_events(&display, &mut events_loop);
}


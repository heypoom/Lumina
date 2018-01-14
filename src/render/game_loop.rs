use glium::Display;
use glium::glutin::{Event, EventsLoop, WindowEvent};

use super::Game;
use super::Camera;
use super::input::Input;

pub fn run(display: &Display, event_loop: &mut EventsLoop) {
  let mut game = Game::new(display);
  let mut active = true;

  while active {
    game.update();

    event_loop.poll_events(|event| {
      match event {
        Event::WindowEvent { event, .. } => {
          match event {
            WindowEvent::Closed => active = false,
            WindowEvent::KeyboardInput { input, .. } => {
              game.handle_keyboard(input.scancode);

              let Camera {x, y, z, pitch, yaw} = game.camera;
              let debug_str = format!("Lumina v0.1 | x {}, y {}, z {}, pitch {}, yaw {}", x, y, z, pitch, yaw);
              game.display.gl_window().set_title(&debug_str);
            },
            _ => (),
          }
        },
        _ => {}
      }
    });
  }
}


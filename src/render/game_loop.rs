use glium::Display;
use glium::glutin::*;

use super::Game;
use super::Camera;

pub struct GameLoop {
  pub game: Game,
  pub event_loop: EventsLoop,
  pub active: bool
}

impl GameLoop {
  pub fn new(display: &Display, event_loop: EventsLoop) -> GameLoop {
    GameLoop {
      game: Game::new(&display),
      event_loop,
      active: true
    }
  }

  pub fn run(&self) {
    while self.active {
      self.event_loop.poll_events(|event| {
        match event {
          Event::WindowEvent { event, .. } => self.handle_window(event),
          _ => {}
        }
      })
    }
  }

  pub fn handle_window(&mut self, event: Event) {
    match event {
      WindowEvent::Closed => self.closed = true,
      WindowEvent::KeyboardInput { input, .. } => {
        self.debug_title()
      },
      _ => (),
    }
  }

  pub fn debug_title(&self) {
    let Camera {x, y, z, pitch, yaw} = self.game.camera;
    let debug_str = format!("Lumina v0.1 | x {}, y {}, z {}, pitch {}, yaw {}", x, y, z, pitch, yaw);
    self.game.display.gl_window().set_title(&debug_str);
  }
}

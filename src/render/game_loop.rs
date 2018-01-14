use glium::Display;
use glium::glutin::{Event, EventsLoop, WindowEvent};

use super::Game;
use super::Camera;
use super::input;

pub struct GameLoop<'a> {
  pub game: Game<'a>,
  pub event_loop: EventsLoop,
  pub active: bool
}

impl<'a> GameLoop<'a> {
  pub fn new(display: &Display, event_loop: EventsLoop) -> GameLoop {
    let mut game_loop = GameLoop {
      game: Game::new(display),
      event_loop,
      active: true
    };

    game_loop.run();

    game_loop
  }

  pub fn run(&mut self) {
    while self.active {
      self.event_loop.poll_events(|event| {
        match event {
          Event::WindowEvent { event, .. } => {
            &self.handle_window(event);
          },
          _ => {}
        };
      });
    }
  }

  pub fn handle_window(&mut self, event: WindowEvent) {
    match event {
      WindowEvent::Closed => self.close(),
      WindowEvent::KeyboardInput { input, .. } => {
        input::handle_keyboard(input.scancode, &mut self.game);
        self.debug_title();
      },
      _ => (),
    }
  }

  pub fn close(&mut self) {
    self.active = false
  }

  pub fn debug_title(&self) {
    let Camera {x, y, z, pitch, yaw} = self.game.camera;
    let debug_str = format!("Lumina v0.1 | x {}, y {}, z {}, pitch {}, yaw {}", x, y, z, pitch, yaw);
    self.game.display.gl_window().set_title(&debug_str);
  }
}

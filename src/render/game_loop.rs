use glium::Display;
use glium::glutin::{Event, EventsLoop, WindowEvent};

use std::rc::Rc;
use std::cell::RefCell;

use super::Game;
use super::Camera;
use super::input::Input;

pub struct GameLoop<'a> {
  pub game: Game<'a>,
  pub event_loop: EventsLoop
}

impl<'a> GameLoop<'a> {
  pub fn new(display: &Display, event_loop: EventsLoop) -> GameLoop {
    let mut game_loop = GameLoop {
      game: Game::new(display),
      event_loop
    };

    game_loop.run();

    game_loop
  }

  pub fn run(&mut self) {
    let mut active = true;

    let this = Rc::new(RefCell::new(self));
    let s1 = Rc::clone(&this);
    let s2 = Rc::clone(&this);

    // FIXME: Make this not crash!
    let mut ref_s1 = s1.borrow_mut();
    let mut ref_s2 = s2.borrow_mut();

    while active {
      ref_s1.event_loop.poll_events(|event| {
        match event {
          Event::WindowEvent { event, .. } => {
            match event {
              WindowEvent::Closed => active = false,
              WindowEvent::KeyboardInput { input, .. } => {
                ref_s2.game.handle_keyboard(input.scancode);
                ref_s2.debug_title();
              },
              _ => (),
            }
          },
          _ => {}
        }
      });
    }
  }

  pub fn debug_title(&self) {
    let Camera {x, y, z, pitch, yaw} = self.game.camera;
    let debug_str = format!("Lumina v0.1 | x {}, y {}, z {}, pitch {}, yaw {}", x, y, z, pitch, yaw);
    self.game.display.gl_window().set_title(&debug_str);
  }
}

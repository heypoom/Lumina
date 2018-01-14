use glium::Display;
use glium::glutin::{Event, EventsLoop, WindowEvent};

use std::rc::Rc;
use std::cell::RefCell;

use super::Game;
use super::Camera;
use super::input::Input;

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
    let this = Rc::new(RefCell::new(self));
    let s2 = Rc::clone(&this);
    let s3 = Rc::clone(&this);
    let s4 = Rc::clone(&this);

    while s2.borrow().active {
      println!("Active");

      s3.borrow_mut().event_loop.poll_events(|event| {
        println!("Polling");

        match event {
          Event::WindowEvent { event, .. } => {
            s4.borrow_mut().handle_window(event);
          },
          _ => {}
        }
      });
    }
  }

  pub fn handle_window(&mut self, event: WindowEvent) {
    match event {
      WindowEvent::Closed => self.close(),
      WindowEvent::KeyboardInput { input, .. } => {
        self.game.handle_keyboard(input.scancode);
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

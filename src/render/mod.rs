extern crate glium;
extern crate image;

use self::glium::Display;
use self::glium::glutin::*;

mod game;
mod game_loop;
mod cube;
mod shader;
mod texture;
mod matrix;
mod input;
mod camera;

pub use self::game_loop::GameLoop;
pub use self::camera::Camera;
pub use self::game::Game;

pub const PI: f32 = 3.141592;

pub fn init() {
  let mut events_loop = EventsLoop::new();

  let window = WindowBuilder::new()
    .with_dimensions(1024, 768)
    .with_title("Lumina 0.1");

  let context = ContextBuilder::new().with_depth_buffer(24);

  let display = Display::new(window, context, &events_loop).unwrap();

  GameLoop::new(&display, events_loop);
}

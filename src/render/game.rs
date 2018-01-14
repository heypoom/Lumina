extern crate glium;

use glium::{Display, Program};

use super::shader;
use super::camera::Camera;
use super::texture::Texture;
use super::renderer::Renderer;

pub struct Game<'a> {
  pub camera: Camera,
  pub display: &'a Display,
  pub tick: f32,
  pub texture: Texture,
  pub shader: Program
}

impl<'a> Game<'a> {
  pub fn new(display: &Display) -> Game {
    let texture = Texture::load("prismarine_bricks", display);
    let shader = shader::load_shader("Basic", display);

    Game {
      camera: Camera::new(),
      display,
      tick: 0.0,
      shader,
      texture
    }
  }

  pub fn update(&mut self) {
    self.tick += 0.0002;

    if self.tick > 0.5 {
      self.tick = -0.5;
    }

    self.render();
  }
}


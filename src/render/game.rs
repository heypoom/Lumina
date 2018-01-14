extern crate glium;

use glium::{Display, Program, Surface, DrawParameters};
use super::camera::Camera;
use super::texture::Texture;
use super::cube;
use super::shader;

pub struct Game<'a> {
  pub camera: Camera,
  pub display: &'a Display,
  pub tick: f32,
  pub texture: Texture,
  pub shader: Program
}

static draw_params: DrawParameters = DrawParameters {
  depth: glium::Depth {
    test: glium::draw_parameters::DepthTest::IfLess,
    write: true,
    ..Default::default()
  },
  backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
  ..Default::default()
};

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

  pub fn render(&mut self) {
    let t = self.tick;

    let model = [
      [t.cos(), t.sin(), 0.0, 0.0],
      [-t.sin(), t.cos(), 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0f32],
    ];

    let mut target = self.display.draw();

    let (model, perspective) = self.camera.render(&target);
    let shape = cube::new_cube(self.display);

    // let light = [1.7, 0.3, 0.7f32];
    let light = [-1.0, 0.4, 0.9f32];

    let uniforms = uniform! {
      diffuse_tex: &self.texture.diffuse,
      normal_tex: &self.texture.normal,
      specular_tex: &self.texture.specular,
      u_light: light
    };

    let bg_color = (0.93, 0.93, 0.93, 1.0);

    target.clear_color_and_depth(bg_color, 1.0);

    target.draw(&shape, &indices, &shader, &uniforms, &draw_params).unwrap();

    target.finish().unwrap();
  }
}

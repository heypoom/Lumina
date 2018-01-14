use glium::{Surface, Depth, DrawParameters};
use glium::draw_parameters::{DepthTest, BackfaceCullingMode};

use super::Game;
use super::cube;

pub trait Renderer {
  fn render(&mut self);
}

impl<'a> Renderer for Game<'a> {
  fn render(&mut self) {
    let t = self.tick;

    let model = [
      [t.cos(), t.sin(), 0.0, 0.0],
      [-t.sin(), t.cos(), 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0f32],
    ];

    let mut target = self.display.draw();

    let (view, perspective) = self.camera.render(&target);
    let (shape, indices) = cube::new_cube(self.display);

    // let light = [1.7, 0.3, 0.7f32];
    let light = [-1.0, 0.4, 0.9f32];

    let uniforms = uniform! {
      model: model,
      view: view,
      perspective: perspective,
      diffuse_tex: &self.texture.diffuse,
      normal_tex: &self.texture.normal,
      specular_tex: &self.texture.specular,
      u_light: light
    };

    let bg_color = (0.93, 0.93, 0.93, 1.0);

    let params = DrawParameters {
      depth: Depth {
        test: DepthTest::IfLess,
        write: true,
        ..Default::default()
      },
      backface_culling: BackfaceCullingMode::CullClockwise,
      ..Default::default()
    };

    target.clear_color_and_depth(bg_color, 1.0);

    target.draw(&shape, &indices, &self.shader, &uniforms, &params).unwrap();

    target.finish().unwrap();
  }
}

extern crate glium;
extern crate image;

use self::glium::{index, Display, IndexBuffer, Program, Surface, VertexBuffer};
use self::glium::glutin::*;
use self::glium::backend::Facade;
use self::glium::texture::{RawImage2d, SrgbTexture2d, Texture2d};

mod cube;
mod shader;
mod texture;
mod matrix;
mod camera;

use self::camera::Camera;

pub const PI: f32 = 3.141592;

fn handle_events(display: Display, events_loop: &mut EventsLoop) {
  let mut closed = false;
  let cam = Camera::new();

  let shader = shader::get_shader("Basic", &display);

  let (diffuse, normal_map, specular_map) = texture::load_texture("prismarine_bricks", &display);

  let mut t: f32 = -0.5;

  while !closed {
    t += 0.0002;

    if t > 0.5 {
      t = -0.5;
    }

    let model = [
      [t.cos(), t.sin(), 0.0, 0.0],
      [-t.sin(), t.cos(), 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0f32],
    ];

    let mut target = display.draw();

    let (model, perspective) = cam.render(&target);

    // let light = [1.7, 0.3, 0.7f32];
    let light = [-1.0, 0.4, 0.9f32];

    let uniforms = uniform! {
      diffuse_tex: &diffuse,
      normal_tex: &normal_map,
      specular_tex: &specular_map,
      u_light: light
    };

    let params = glium::DrawParameters {
      depth: glium::Depth {
        test: glium::draw_parameters::DepthTest::IfLess,
        write: true,
        ..Default::default()
      },
      // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
      ..Default::default()
    };

    let bg_color = (0.93, 0.93, 0.93, 1.0);

    target.clear_color_and_depth(bg_color, 1.0);

    target.draw(&shape, &indices, &shader, &uniforms, &params).unwrap();

    target.finish().unwrap();

    events_loop.poll_events(|ev| match ev {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Closed => closed = true,
        WindowEvent::KeyboardInput { input, .. } => {
          let debug_str = format!("Lumina v0.1 | x {}, y {}, z {}, pitch {}, yaw {}", x, y, z, pitch, yaw);
          &display.gl_window().set_title(&debug_str);


        },
        _ => (),
      },
      _ => (),
    });
  }
}

pub fn init() {
  let mut events_loop = EventsLoop::new();

  let window = WindowBuilder::new()
    .with_dimensions(1024, 768)
    .with_title("Lumina 0.1");

  let context = ContextBuilder::new().with_depth_buffer(24);

  let display = Display::new(window, context, &events_loop).unwrap();

  handle_events(display, &mut events_loop);
}

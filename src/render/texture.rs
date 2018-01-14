extern crate image;

use glium::backend::Facade;
use glium::texture::{RawImage2d, SrgbTexture2d, Texture2d};

use std::fs::File;
use std::io::BufReader;

pub struct Texture {
  pub diffuse: SrgbTexture2d,
  pub normal: Texture2d,
  pub specular: Texture2d
}

pub fn raw_texture(name: &str) -> RawImage2d<u8> {
  let image_path = format!("./textures/{}.png", name);
  let image_file = File::open(image_path).unwrap();
  let image_reader = BufReader::new(image_file);

  let image = image::load(image_reader, image::PNG).unwrap().to_rgba();
  let dimensions = image.dimensions();
  let raw: &[u8] = &image.into_raw();

  RawImage2d::from_raw_rgba_reversed(raw, dimensions)
}

impl Texture {
  pub fn load<T: Facade>(name: &str, display: &T) -> Texture {
    let normal_name = format!("{}_n", name);
    let specular_name = format!("{}_s", name);

    let diffuse = SrgbTexture2d::new(display, raw_texture(name)).unwrap();
    let normal = Texture2d::new(display, raw_texture(&normal_name)).unwrap();
    let specular = Texture2d::new(display, raw_texture(&specular_name)).unwrap();

    Texture {diffuse, normal, specular}
  }
}

extern crate image;

use glium::{index, Display, IndexBuffer, Program, Surface, VertexBuffer};
use glium::backend::Facade;
use glium::texture::{RawImage2d, SrgbTexture2d, Texture2d};

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn raw_texture(name: &str) -> RawImage2d<u8> {
  let image_path = format!("./textures/{}.png", name);
  let image_file = File::open(image_path).unwrap();
  let image_reader = BufReader::new(image_file);

  let image = image::load(image_reader, image::PNG).unwrap().to_rgba();
  let dimensions = image.dimensions();
  let raw: &[u8] = &image.into_raw();

  RawImage2d::from_raw_rgba_reversed(raw, dimensions)
}

pub fn load_texture<T: Facade>(name: &str, display: &T) -> (SrgbTexture2d, Texture2d, Texture2d) {
  let normal_name = format!("{}_n", name);
  let specular_name = format!("{}_s", name);

  let diffuse = SrgbTexture2d::new(display, raw_texture(name)).unwrap();
  let normal = Texture2d::new(display, raw_texture(&normal_name)).unwrap();
  let specular = Texture2d::new(display, raw_texture(&specular_name)).unwrap();

  (diffuse, normal, specular)
}

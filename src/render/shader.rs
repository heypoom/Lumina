use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use glium::{index, Display, IndexBuffer, Program, Surface, VertexBuffer};
use glium::backend::Facade;

pub fn get_shader<T: Facade>(name: &str, display: &T) -> Program {
  let mut vert_src = String::new();
  let mut frag_src = String::new();

  let vert_path = format!("./shaders/{}.vert", name);
  let frag_path = format!("./shaders/{}.frag", name);

  let mut vert_file = File::open(vert_path).unwrap();
  let mut frag_file = File::open(frag_path).unwrap();

  vert_file.read_to_string(&mut vert_src).unwrap();
  frag_file.read_to_string(&mut frag_src).unwrap();

  Program::from_source(display, &vert_src, &frag_src, None).unwrap()
}

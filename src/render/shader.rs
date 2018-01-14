use std::fs::File;
use std::io::prelude::*;

use glium::Program;
use glium::backend::Facade;

static SHADER_PATH: &str = "./shaders/";

pub fn get_shader<T: Facade>(name: &str, display: &T) -> Program {
  let mut vert_src = String::new();
  let mut frag_src = String::new();

  let vert_path = format!("{}{}.vert", SHADER_PATH, name);
  let frag_path = format!("{}{}.frag", SHADER_PATH, name);

  let mut vert_file = File::open(vert_path).unwrap();
  let mut frag_file = File::open(frag_path).unwrap();

  vert_file.read_to_string(&mut vert_src).unwrap();
  frag_file.read_to_string(&mut frag_src).unwrap();

  Program::from_source(display, &vert_src, &frag_src, None).unwrap()
}

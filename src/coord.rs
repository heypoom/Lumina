#[derive(Debug)]
pub struct Coord {
  x: u32,
  y: u32,
  z: u32
}

impl Coord {
  pub fn new(x: u32, y: u32, z: u32) -> Coord {
    Coord {x, y, z}
  }

  pub fn blank() -> Coord {
    Coord {x: 0, y: 0, z: 0}
  }
}

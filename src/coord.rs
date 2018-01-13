#[derive(Debug, Clone)]
pub struct Coord {
  pub x: u32,
  pub y: u32,
  pub z: u32
}

impl Coord {
  pub fn new(x: u32, y: u32, z: u32) -> Coord {
    Coord {x, y, z}
  }

  pub fn blank() -> Coord {
    Coord {x: 0, y: 0, z: 0}
  }

  pub fn set(&mut self, x: u32, y: u32, z: u32) {
    self.x = x;
    self.y = y;
    self.z = z;
  }
}

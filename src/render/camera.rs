pub struct Camera {
  pub x: f32,
  pub y: f32,
  pub z: f32,

  // Vertical (Up/Down) Angle
  pub pitch: f32,

  // Horizontal (Left/Right) Angle
  pub yaw: f32
}

impl Camera {
  pub fn new() -> Camera {
    Camera {
      x: -3.5,
      y: -0.5,
      z: 4.5,
      pitch: -3.89,
      yaw: 6.5
    }
  }

  pub fn add_x(&mut self, val: f32) {
    self.x += val;
  }

  pub fn add_y(&mut self, val: f32) {
    self.y += val;
  }

  pub fn add_z(&mut self, val: f32) {
    self.z += val;
  }

  pub fn add_pitch(&mut self, val: f32) {
    self.pitch += val;
  }

  pub fn add_yaw(&mut self, val: f32) {
    self.yaw += val;
  }
}

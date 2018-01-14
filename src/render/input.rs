const MOVE_FORWARD = 13;
const MOVE_LEFT = 0;
const MOVE_BACKWARD = 1;
const MOVE_RIGHT = 2;
const JUMP = 49;
const CROUCH = 56;

const ARROW_UP = 126
const ARROW_DOWN = 125
const ARROW_LEFT = 123
const ARROW_RIGHT = 124

// +; 24 => {}
// -; 27 => {}
// J; 38 => {}
// K; 40 => {}

pub fn handle_keyboard(scancode: u32) {
  // TODO: Get Camera

  match scancode {
    MOVE_FORWARD => {
      camera.add_z(-0.1)
    }
    MOVE_LEFT => {
       camera.add_x(0.1)
    }
    MOVE_BACKWARD => {
      camera.add_z(0.1)
    }
    MOVE_RIGHT => {
      camera.add_x(-0.1)
    }
    JUMP => {
      camera.add_y(0.1)
    }
    CROUCH => {
      camera.add_y(-0.1)
    }
    ARROW_UP => {
      camera.add_pitch(0.1)
    }
    ARROW_DOWN => {
      camera.add_pitch(-0.1)
    }
    ARROW_LEFT => {
      camera.add_yaw(-0.1)
    }
    ARROW_RIGHT => {
      camera.add_yaw(0.1)
    }
    _ => println!("Unmapped Key: {}", &input.scancode)
  }
}

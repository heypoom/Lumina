use glium::{IndexBuffer, VertexBuffer};
use glium::index::PrimitiveType;
use glium::backend::Facade;

#[derive(Copy, Clone)]
pub struct Vertex {
  pub pos: [f32; 3],
  pub normal: [f32; 3],
  pub uv: [f32; 2],
}

implement_vertex!(Vertex, pos, normal, uv);

pub fn new_cube<T: Facade>(display: &T) -> (VertexBuffer<Vertex>, IndexBuffer<u16>) {
  #[cfg_attr(rustfmt, rustfmt_skip)]
  let shape = VertexBuffer::new(display, &[
    // Front
    Vertex { pos: [-1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [1.0, 0.0] },
    Vertex { pos: [ 1.0, -1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [0.0, 0.0] },
    Vertex { pos: [ 1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [0.0, 1.0] },
    Vertex { pos: [-1.0,  1.0,  1.0], normal: [ 0.0,  0.0,  1.0], uv: [1.0, 1.0] },

    // Right
    Vertex { pos: [ 1.0, -1.0,  1.0], normal: [ 1.0,  0.0,  0.0], uv: [1.0, 1.0] },
    Vertex { pos: [ 1.0, -1.0, -1.0], normal: [ 1.0,  0.0,  0.0], uv: [1.0, 0.0] },
    Vertex { pos: [ 1.0,  1.0, -1.0], normal: [ 1.0,  0.0,  0.0], uv: [0.0, 0.0] },
    Vertex { pos: [ 1.0,  1.0,  1.0], normal: [ 1.0,  0.0,  0.0], uv: [0.0, 1.0] },

    // Back
    Vertex { pos: [-1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [0.0, 0.0] },
    Vertex { pos: [-1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [0.0, 1.0] },
    Vertex { pos: [ 1.0,  1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [1.0, 1.0] },
    Vertex { pos: [ 1.0, -1.0, -1.0], normal: [ 0.0,  0.0, -1.0], uv: [1.0, 0.0] },

    // Left
    Vertex { pos: [-1.0, -1.0,  1.0], normal: [-1.0,  0.0,  0.0], uv: [0.0, 1.0] },
    Vertex { pos: [-1.0,  1.0,  1.0], normal: [-1.0,  0.0,  0.0], uv: [1.0, 1.0] },
    Vertex { pos: [-1.0,  1.0, -1.0], normal: [-1.0,  0.0,  0.0], uv: [1.0, 0.0] },
    Vertex { pos: [-1.0, -1.0, -1.0], normal: [-1.0,  0.0,  0.0], uv: [0.0, 0.0] },

    // Bottom
    Vertex { pos: [-1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0], uv: [0.0, 1.0] },
    Vertex { pos: [-1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0], uv: [0.0, 0.0] },
    Vertex { pos: [ 1.0, -1.0, -1.0], normal: [ 0.0, -1.0,  0.0], uv: [1.0, 0.0] },
    Vertex { pos: [ 1.0, -1.0,  1.0], normal: [ 0.0, -1.0,  0.0], uv: [1.0, 1.0] },

    // Top
    Vertex { pos: [-1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0], uv: [0.0, 1.0] },
    Vertex { pos: [ 1.0,  1.0,  1.0], normal: [ 0.0,  1.0,  0.0], uv: [0.0, 1.0] },
    Vertex { pos: [ 1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0], uv: [1.0, 0.0] },
    Vertex { pos: [-1.0,  1.0, -1.0], normal: [ 0.0,  1.0,  0.0], uv: [0.0, 0.0] },
  ]).unwrap();

  #[cfg_attr(rustfmt, rustfmt_skip)]
  let indices_list = &[
    // Front
    0u16, 2, 1, 0, 3, 2,

    // Right
    4, 6, 5, 4, 7, 6,

    // Back
    8, 10, 9, 8, 11, 10,

    // Left
    12, 14, 13, 12, 15, 14,

    // Bottom
    16, 18, 17, 16, 19, 18,

    // Top
    20, 22, 21, 20, 23, 22,
  ];

  let indices = IndexBuffer::new(display, PrimitiveType::TrianglesList, indices_list).unwrap();

  (shape, indices)
}

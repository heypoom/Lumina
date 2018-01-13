extern crate lumina;

use lumina::*;

fn main() {
  let Grass = VoxelType::new("voxel:grass").name("Grass");

  let mut grass = Grass.create();
  grass.move_to(5, 200, 5);

  let mut Stone = VoxelType::new("voxel:stone")
    .name("Stone")
    .hardness(10)
    .brightness(5)
    .texture("voxel_stone");

  let mut stone = Stone.create();
  &stone.move_to(2, 50, 2);

  let Glowstone = Stone
    .name("Glowstone")
    .texture("voxel_glowstone")
    .brightness(20);

  let mut glowstone = Glowstone.create();
  glowstone.move_to(70, 100, 70);

  let blocks: Vec<VoxelInstance> = vec![grass, stone, glowstone];
  println!("{:?}", blocks);
}


extern crate lumina;

use lumina::*;

fn main() {
  let mut Grass = VoxelType::new("voxel:grass");
  Grass.name("Grass");

  let mut grass = Grass.create();
  &grass.move_to(5, 200, 5);

  let mut Stone = VoxelType::new("voxel:stone");
  Stone.name("Stone");
  Stone.hardness(10);
  Stone.brightness(5);
  Stone.texture("voxel_stone");

  let mut stone = Stone.create();
  &stone.move_to(2, 50, 2);

  let mut Glowstone = Stone.clone();
  Glowstone.name("Glowstone");
  Glowstone.texture("voxel_glowstone");
  Glowstone.brightness(20);

  let mut glowstone = Glowstone.create();
  &glowstone.move_to(70, 100, 70);

  let blocks: Vec<VoxelInstance> = vec![grass, stone, glowstone];
  println!("{:?}", blocks);
}


extern crate lumina;

use lumina::*;

fn main() {
    let Grass = Voxel::new("voxel:grass").name("Grass");
    let grass = Grass.create().moveTo(5, 200, 5);

    let Stone = Voxel::new("voxel:stone")
        .name("Stone")
        .hardness(10)
        .brightness(5)
        .texture("voxel_stone");

    let mut stone = Stone::new();
    &stone.moveTo(2, 50, 2);

    let Glowstone = Stone
        .name("Glowstone")
        .texture("voxel_glowstone")
        .brightness(20);

    let glowstone = Glowstone::new().moveTo(0, 0, 0);

    let blocks: Vec<Voxel> = vec![grass, stone, ];
    println!("{:?}", blocks.get(0).unwrap());
}


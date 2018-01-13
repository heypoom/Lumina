use coord::Coord;

/// What will happen when we place this voxel?
pub trait Place {}

/// What will happen when we destroy this voxel?
pub trait Destroy {}

/// What will happen when we interact with this voxel with Right Click?
pub trait Interact {}

/// VoxelType defines the many different Types of Voxel
#[derive(Debug, Clone)]
pub struct VoxelType {
	id: String,
	name: String,
	texture: String,
	model: String,
	hardness: u32,
	brightness: u32,
}

static MISSING_TEXTURE: &str = "missing_texture";
static BLOCK_MODEL: &str = "block";

impl VoxelType {
	/// Creates a Voxel Type Builder, which allows chaining of Properties in the VoxelType.
	pub fn new(id: &str) -> VoxelType {
		VoxelType {
			id: String::from(id),
			name: String::from("Unnamed Voxel"),
			texture: String::from(MISSING_TEXTURE),
			model: String::from(BLOCK_MODEL),
			hardness: 5,
			brightness: 5
		}
	}

	/// Creates an instance of a Voxel
	pub fn create(&self) -> VoxelInstance {
		VoxelInstance::new(self.clone())
	}

  pub fn name(&mut self, name: &str) -> &mut Self {
    self.name = name.to_string();
    self
  }

  pub fn texture(&mut self, src: &str) -> &mut Self {
    self.texture = src.to_string();
    self
  }

  pub fn hardness(&mut self, level: u32) -> &mut Self {
    self.hardness = level;
    self
  }

  pub fn brightness(&mut self, level: u32) -> &mut Self {
    self.brightness = level;
    self
  }
}

#[derive(Debug, Clone)]
pub struct VoxelInstance {
	pub kind: VoxelType,
	pub pos: Coord
}

impl VoxelInstance {
	/// Returns an instance of a Voxel
	pub fn new(kind: VoxelType) -> VoxelInstance {
		VoxelInstance {
			kind,
			pos: Coord::blank()
		}
	}

  pub fn move_to(&mut self, x: u32, y: u32, z: u32) {
    self.pos.set(x, y, z);
  }
}


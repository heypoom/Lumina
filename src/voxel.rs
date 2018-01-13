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
		VoxelInstance::new(self)
	}

  pub fn name(&self, name: &str) -> Self {
    let mut v = self.clone();
    v.name = name.to_string();
    v
  }

  pub fn texture(&self, src: &str) -> Self {
    let mut v = self.clone();
    v.texture = src.to_string();
    v
  }

  pub fn hardness(&self, level: u32) -> Self {
    let mut v = self.clone();
    v.hardness = level;
    v
  }

  pub fn brightness(&self, level: u32) -> Self {
    let mut v = self.clone();
    v.brightness = level;
    v
  }
}

#[derive(Debug, Clone)]
pub struct VoxelInstance<'a> {
	pub kind: &'a VoxelType,
	pub pos: Coord
}

impl<'a> VoxelInstance<'a> {
	/// Returns an instance of a Voxel
	pub fn new(kind: &'a VoxelType) -> VoxelInstance {
		VoxelInstance {
			kind,
			pos: Coord::blank()
		}
	}

  pub fn move_to(&mut self, x: u32, y: u32, z: u32) {
    self.pos.set(x, y, z);
  }
}


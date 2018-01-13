extern crate lumina;

/// What will happen when we place this voxel?
pub trait Place {}

/// What will happen when we destroy this voxel?
pub trait Destroy {}

/// What will happen when we interact with this voxel with Right Click?
pub trait Interact {}

/// VoxelType defines the many different Types of Voxel
struct VoxelType {
    id: String,
    name: String,
    texture: String,
    model: String,
    hardness: u32,
    brightness: u32,
}

static MISSING_TEXTURE: &str = String::from("missing_texture")
static BLOCK_MODEL: &str = String::from("block");

impl VoxelType {
    /// Creates a Voxel Type Builder, which allows chaining of Properties in the VoxelType.
    pub fn new(id: &str) -> VoxelType {
        VoxelType {
            id: id.to_string(),
            name: String::from("Unnamed Voxel"),
            texture: MISSING_TEXTURE,
            model: BLOCK_MODEL,
            hardness: 5,
            brightness: 5
        }
    }

    /// Creates an instance of a Voxel
    pub fn create(&self) -> VoxelInstance {
        VoxelInstance::new(*self)
    }

    pub fn name(&self, name: String) -> VoxelType {
        let voxel = *self.clone();
        voxel.name = name;
        voxel
    }
}

struct VoxelInstance {
    kind: VoxelType,
    position: Coord
}

impl VoxelInstance {
    /// Returns an instance of a Voxel
    pub fn new(kind: VoxelType) -> VoxelInstance {
        VoxelInstance {
            kind,
            coord: Coord::blank()
        }
    }
}


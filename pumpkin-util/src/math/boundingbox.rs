use super::{position::BlockPos, vector3::Vec3};

#[derive(Clone, Copy, Debug)]
pub struct BoundingBox {
    pub min: Vec3<f64>,
    pub max: Vec3<f64>,
}

impl BoundingBox {
    pub fn new_default(size: &BoundingBoxSize) -> Self {
        Self::new_from_pos(0., 0., 0., size)
    }

    pub fn new_from_pos(x: f64, y: f64, z: f64, size: &BoundingBoxSize) -> Self {
        let f = size.width / 2.;
        Self {
            min: Vec3::new(x - f, y, z - f),
            max: Vec3::new(x + f, y + size.height, z + f),
        }
    }

    pub fn offset(&self, other: Self) -> Self {
        Self {
            min: self.min.add(&other.min),
            max: self.max.add(&other.max),
        }
    }

    pub fn new(min: Vec3<f64>, max: Vec3<f64>) -> Self {
        Self { min, max }
    }

    pub fn new_array(min: [f64; 3], max: [f64; 3]) -> Self {
        Self {
            min: Vec3::new(min[0], min[1], min[2]),
            max: Vec3::new(max[0], max[1], max[2]),
        }
    }

    pub fn from_block(position: &BlockPos) -> Self {
        let position = position.0;
        Self {
            min: Vec3::new(position.x as f64, position.y as f64, position.z as f64),
            max: Vec3::new(
                position.x as f64 + 1.0,
                position.y as f64 + 1.0,
                position.z as f64 + 1.0,
            ),
        }
    }

    pub fn from_block_raw(position: &BlockPos) -> Self {
        let position = position.0;
        Self {
            min: Vec3::new(position.x as f64, position.y as f64, position.z as f64),
            max: Vec3::new(position.x as f64, position.y as f64, position.z as f64),
        }
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }

    pub fn squared_magnitude(&self, pos: Vec3<f64>) -> f64 {
        let d = f64::max(f64::max(self.min.x - pos.x, pos.x - self.max.x), 0.0);
        let e = f64::max(f64::max(self.min.y - pos.y, pos.y - self.max.y), 0.0);
        let f = f64::max(f64::max(self.min.z - pos.z, pos.z - self.max.z), 0.0);
        super::squared_magnitude(d, e, f)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BoundingBoxSize {
    pub width: f64,
    pub height: f64,
}

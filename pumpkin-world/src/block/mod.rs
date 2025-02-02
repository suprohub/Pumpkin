pub mod registry;
pub mod state;

use num_derive::FromPrimitive;
use pumpkin_util::math::vector3::Vec3;

pub use state::BlockState;

#[derive(FromPrimitive, PartialEq, Clone, Copy)]
pub enum BlockDirection {
    Bottom = 0,
    Top,
    North,
    South,
    West,
    East,
}

pub struct InvalidBlockFace;

impl TryFrom<i32> for BlockDirection {
    type Error = InvalidBlockFace;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Bottom),
            1 => Ok(Self::Top),
            2 => Ok(Self::North),
            3 => Ok(Self::South),
            4 => Ok(Self::West),
            5 => Ok(Self::East),
            _ => Err(InvalidBlockFace),
        }
    }
}

impl BlockDirection {
    pub fn to_offset(&self) -> Vec3<i32> {
        match self {
            BlockDirection::Bottom => (0, -1, 0),
            BlockDirection::Top => (0, 1, 0),
            BlockDirection::North => (0, 0, -1),
            BlockDirection::South => (0, 0, 1),
            BlockDirection::West => (-1, 0, 0),
            BlockDirection::East => (1, 0, 0),
        }
        .into()
    }
    pub fn opposite(&self) -> BlockDirection {
        match self {
            BlockDirection::Bottom => BlockDirection::Top,
            BlockDirection::Top => BlockDirection::Bottom,
            BlockDirection::North => BlockDirection::South,
            BlockDirection::South => BlockDirection::North,
            BlockDirection::West => BlockDirection::East,
            BlockDirection::East => BlockDirection::West,
        }
    }
}

use crate::AABB;
use nalgebra::Vector3;

pub enum BlockFace {
    Up,
    Down,
    North,
    South,
    East,
    West,
}

impl BlockFace {
    pub fn opposite(&self) -> BlockFace {
        match self {
            BlockFace::Up       => BlockFace::Down,
            BlockFace::Down     => BlockFace::Up,
            BlockFace::North    => BlockFace::South,
            BlockFace::South    => BlockFace::North,
            BlockFace::East     => BlockFace::West,
            BlockFace::West     => BlockFace::East,
        }
    }

    pub fn coord(c: usize) -> BlockFace {
        match c {
            0 => BlockFace::East,
            1 => BlockFace::Up,
            _ => BlockFace::North,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(u32)]
pub enum Block {
    Air,
    Water,
    Stone,
    Grass,
    Dirt,
    Gravel,

    Sand,
    Cactus,
    Snow,
    TallGrass,

    OakLog,
    AcaciaLog,
    BigOakLog,
    BirchLog,
    JungleLog,
    SpruceLog,
    OakLeaves,
    AcaciaLeaves,
    BigOakLeaves,
    BirchLeaves,
    JungleLeaves,
    SpruceLeaves,
}

impl Block {
    pub fn is_tough(&self) -> bool {
        match self {
            Block::Air | Block::Snow => false,
            Block::TallGrass => false,
            _ => true,
        }
    }

    pub fn is_liquid(&self) -> bool {
        match self {
            Block::Water => true,
            _ => false,
        }
    }

    pub fn aabb(&self, position: Vector3<f32>) -> Option<AABB> {
        let base = match self {
            Block::Air | Block::Water => None,
            Block::Snow => Some(AABB::new(Vector3::zeros(), Vector3::new(1., 0.1, 1.))),
            _ => Some(AABB::new(Vector3::zeros(), Vector3::new(1., 1., 1.))),
        };

        if let Some(base) = base {
            Some(base.translate(position))
        } else {
            None
        }
    }
}

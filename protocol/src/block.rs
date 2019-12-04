#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockCode {
    SimpleSlab = 1,
    SimpleWall,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Orientation {
    Up = 1,
    Down,
    Left,
    Right
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Block {
    block_code: BlockCode,
    orientation: Orientation
}

impl Block {
    pub fn new(block_code: BlockCode, orientation: Orientation) -> Self {
        Self {
            block_code,
            orientation
        }
    }

    pub fn get_block_code(self) -> BlockCode {
        self.block_code
    }

    pub fn set_block_code(&mut self, block_code: BlockCode) {
        self.block_code = block_code;
    }

    pub fn get_orientation(self) -> Orientation {
        self.orientation
    }

    pub fn set_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Chunk {
    pub x: u64,
    pub y: u64,
    pub blocks: [[Block;8];8]
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new(9_223_372_036_854_775_808, 9_223_372_036_854_775_808, [[Block::default();8];8])
    }
}

impl Chunk {
    pub fn new(x: u64, y: u64, blocks: [[Block;8];8]) -> Self {
        Self {
            x,
            y,
            blocks
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Self {
            block_code: BlockCode::SimpleSlab,
            orientation: Orientation::Up
        }
    }
}

impl From<u16> for BlockCode {
    fn from(code: u16) -> Self {
        match code {
            1 => BlockCode::SimpleSlab,
            2 => BlockCode::SimpleWall,
            i => panic!("unknow block {}", i),
        }
    }
}

impl From<u8> for Orientation {
    fn from(code: u8) -> Self {
        match code {
            1 => Orientation::Up,
            2 => Orientation::Right,
            3 => Orientation::Down,
            4 => Orientation::Left,
            _ => panic!("unknow orientation"),
        }
    }
}
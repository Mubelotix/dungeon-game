use std::ops::{Index, IndexMut};
use crate::block::{Block, Chunk};

pub struct Map {
    chunks: Vec<Chunk>,
    default_block: Block,
}

impl Map {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            default_block: Block::default()
        }
    }

    pub fn print_info(&self) {
        println!("{} chunks loaded", self.chunks.len());
    }

    pub fn get_chunk(&self, x: u64, y: u64) -> Chunk {
        let x_in_chunk = x % 8;
        let y_in_chunk = y % 8;
        let first_block_x = x - x_in_chunk;
        let first_block_y = y - y_in_chunk;
        let mut idx = 0;
        while idx < self.chunks.len() {
            if self.chunks[idx].x == first_block_x && self.chunks[idx].y == first_block_y {
                return self.chunks[idx];
            }
            idx += 1;
        }
        Chunk::new(x, y, [[Block::default();8];8])
    }

    pub fn set_chunk(&mut self, x: u64, y: u64, blocks: [[Block;8];8]) {
        let x_in_chunk = x % 8;
        let y_in_chunk = y % 8;
        let first_block_x = x - x_in_chunk;
        let first_block_y = y - y_in_chunk;
        let mut idx = 0;
        while idx < self.chunks.len() {
            if self.chunks[idx].x == first_block_x && self.chunks[idx].y == first_block_y {
                self.chunks[idx].blocks = blocks;
                return;
            }
            idx += 1;
        }
        self.chunks.push(Chunk::new(first_block_x, first_block_y * 8, blocks));
    }
}

impl Index<(u64, u64)> for Map {
    type Output = Block;

    fn index(&self, (x, y): (u64, u64)) -> &Self::Output {
        let x_in_chunk = x % 8;
        let y_in_chunk = y % 8;
        let first_block_x = x - x_in_chunk;
        let first_block_y = y - y_in_chunk;
        let mut idx = 0;
        while idx < self.chunks.len() {
            if self.chunks[idx].x == first_block_x && self.chunks[idx].y == first_block_y {
                return &self.chunks[idx].blocks[x_in_chunk as usize][y_in_chunk as usize];
            }
            idx += 1;
        }
        &self.default_block
    }
}

impl IndexMut<(u64, u64)> for Map {
    fn index_mut(&mut self, (x, y): (u64, u64)) -> &mut Self::Output {
        let x_in_chunk = x % 8;
        let y_in_chunk = y % 8;
        let first_block_x = x - x_in_chunk;
        let first_block_y = y - y_in_chunk;
        let mut idx = 0;
        let len = self.chunks.len();
        while idx < len {
            if self.chunks[idx].x == first_block_x && self.chunks[idx].y == first_block_y {
                return &mut self.chunks[idx].blocks[x_in_chunk as usize][y_in_chunk as usize];
            }
            idx += 1;
        }
        self.chunks.push(Chunk::new(first_block_x, first_block_y, [[Block::default();8];8]));
        &mut self.chunks[len].blocks[x_in_chunk as usize][y_in_chunk as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Map;
    use crate::block::*;
	
	#[test]
	fn test() {
        let mut map = Map::new();
        map[(42, 5)] = Block::new(BlockCode::SimpleWall, Orientation::Down);
        map[(42, 4)] = Block::new(BlockCode::SimpleWall, Orientation::Left);
        assert_eq!(map[(42, 5)], Block::new(BlockCode::SimpleWall, Orientation::Down));
        assert_eq!(map[(42, 4)], Block::new(BlockCode::SimpleWall, Orientation::Left));
        assert_eq!(map[(41, 5)], Block::default());

        map.print_info();
	}
}
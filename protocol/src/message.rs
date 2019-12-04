use std::mem::transmute;
use crate::block::{Block, BlockCode, Orientation, Chunk};
use crate::entity::Entity;
use crate::chat::ChatMessage;
use serde::{Serialize, Deserialize};

fn u64_to_8_u8(x: u64) -> [u8;8] {
    unsafe { transmute(x.to_be()) }
}

fn u16_to_2_u8(x: u16) -> [u8; 2] {
    unsafe { transmute(x.to_be()) }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    Connect(String),
    ChatMessage(ChatMessage),
    Chunk(Chunk),
    CreateEntity(Entity)
}

impl Message {
	pub fn decode(data: String) -> Result<Self, &'static str> {
        let data = data.as_bytes();
        if let Ok(message) = bincode::deserialize(&data[..]) {
            Ok(message)
        } else {
            Err("can't deserialise")
        }
	}

	pub fn encode(&self) -> String {
        let data = bincode::serialize(&self).unwrap();
        unsafe {
            String::from_utf8_unchecked(data)
        }
	}
}
use std::mem::transmute;
use crate::block::{Block, BlockCode, Orientation, Chunk};
use crate::entity::Entity;
use serde::{Serialize, Deserialize};

fn u64_to_8_u8(x: u64) -> [u8;8] {
    unsafe { transmute(x.to_be()) }
}

fn u16_to_2_u8(x: u16) -> [u8; 2] {
    unsafe { transmute(x.to_be()) }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    ChatMessage{sender_id: u64, receiver_id: u64, message: String},
    Chunk(Chunk),
    CreateEntity(Entity),
    Init{username: String, screen_width: u32, screen_height: u32, password: Option<String>},
}

impl Message {
	pub fn decode(data: String) -> Result<Self, &'static str> {
        if let Ok(message) = serde_yaml::from_str(&data[..]) {
            Ok(message)
        } else {
            Err("can't deserialise")
        }
	}

	pub fn encode(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }
}
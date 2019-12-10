use std::mem::transmute;
use crate::block::{Block, BlockCode, Orientation, Chunk};
use crate::entity::Entity;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    ChatMessage{sender_id: u64, receiver_id: u64, message: String},
    Chunk(Chunk),
    CreateEntity(Entity),
    Init{username: String, screen_width: u32, screen_height: u32, password: Option<String>},
    Tick,
    MoveEntity{id: u64, lenght: u8, direction: Orientation}
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
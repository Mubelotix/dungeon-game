use crate::block::{Orientation, Chunk};
use crate::entity::Entity;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Message {
    ChatMessage{sender_id: u64, receiver_id: u64, message: String},
    Chunk(Chunk),
    UnloadChunk{x: u64, y: u64},
    CreateEntity(Entity),
    Init{username: String, screen_width: u32, screen_height: u32, password: Option<String>},
    Tick,
    MoveEntity{id: u64, direction: Orientation},
    TpEntity{id: u64, x: u64, y: u64, x2: u8, y2: u8},
}

impl Message {
	pub fn decode(data: String) -> Result<Self, &'static str> {
        if let Ok(message) = serde_yaml::from_str(&data[..]) {
            Ok(message)
        } else {
            Err("can't deserialize")
        }
	}

	pub fn encode(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }
}
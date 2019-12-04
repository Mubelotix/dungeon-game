use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    sender_id: u64,
    receiver_id: u64,
    message: String
}
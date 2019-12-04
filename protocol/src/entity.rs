use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EntityType {
    You,
    Player,
    Ennemy
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    x: u64,
    y: u64,
    id: u64,
    name: String,
    entity_type: EntityType
}
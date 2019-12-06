use serde::{Serialize, Deserialize};
use getrandom::getrandom;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum EntityType {
    You,
    Player,
    Ennemy
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Entity {
    x: u64,
    y: u64,
    id: u64,
    name: String,
    entity_type: EntityType
}

impl Entity {
    pub fn new(x: u64, y: u64, id: u64, name: String, entity_type: EntityType) -> Self {
        Self {
            x,
            y,
            id,
            name,
            entity_type
        }
    }

    pub fn spawn_player(name: String) -> Self {
        let mut id: [u8; 8] = [0; 8];
        getrandom(&mut id).expect("random does not work!");
        let id = u64::from_be_bytes(id);
        Self {
            x: 9_223_372_036_854_775_808,
            y: 9_223_372_036_854_775_808,
            id,
            name,
            entity_type: EntityType::You
        }
    }

    pub fn set_entity_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_entity_type(&self) -> EntityType {
        self.entity_type
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_coords(&self) -> (u64, u64) {
        (self.x, self.y)
    }
}
use serde::{Serialize, Deserialize};
use getrandom::getrandom;
use crate::coords::*;

const CENTER_POINT: u64 = 9_223_372_036_854_775_808;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum EntityType {
    Player,
    Mob
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Entity {
    pub coords: Coords,
    id: u64,
    name: String,
    entity_type: EntityType
}

impl Entity {
    pub fn new(coords: Coords, id: u64, name: String, entity_type: EntityType) -> Self {
        Self {
            coords,
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
            coords: Coords::default(),
            id,
            name,
            entity_type: EntityType::Player
        }
    }

    /*pub fn get_readable_coords(&self) -> (i64, i64) {
        ((self.x as i128 - CENTER_POINT as i128) as i64, (self.y as i128 - CENTER_POINT as i128) as i64)
    }*/

    pub fn set_entity_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_type(&self) -> &EntityType {
        &self.entity_type
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_speed(&self) -> u8 {
        3
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity::spawn_player(String::from("undefined"))
    }
}
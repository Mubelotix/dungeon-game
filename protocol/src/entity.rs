use serde::{Serialize, Deserialize};
use getrandom::getrandom;
use crate::block::Orientation;

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
    x2: u8,
    y2: u8,
    id: u64,
    name: String,
    entity_type: EntityType
}

impl Entity {
    pub fn new(x: u64, y: u64, id: u64, name: String, entity_type: EntityType) -> Self {
        Self {
            x,
            y,
            x2: 0,
            y2: 0,
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
            x2: 0,
            y2: 0,
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

    pub fn set_coords(&mut self, (x, y): (u64, u64)) {
        self.x = x;
        self.y = y;
        self.x2 = 0;
        self.y2 = 0;
    }

    pub fn get_position_in_block(&self) -> (u8, u8) {
        (self.x2, self.y2)
    }

    pub fn get_speed(&self) -> u8 {
        1
    }

    pub fn move_in_direction(&mut self, orientation: Orientation) {
        let in_block_lenght = self.get_speed() % 40;
        let lenght = (self.get_speed() - in_block_lenght) / 40;
        match orientation {
            Orientation::Up => {
                self.y -= lenght as u64;
                if self.y2 < in_block_lenght {
                    self.y -= 1;
                    self.y2 += 40 - in_block_lenght;
                } else {
                    self.y2 -= in_block_lenght;
                }
            },
            Orientation::Down => {
                self.y += lenght as u64;
                if self.y2 + in_block_lenght >= 40  {
                    self.y += 1;
                }
                self.y2 = (self.y2 + in_block_lenght) % 40;
            },
            Orientation::Left => {
                self.x -= lenght as u64;
                if self.x2 < in_block_lenght {
                    self.x -= 1;
                    self.x2 += 40 - in_block_lenght;
                } else {
                    self.x2 -= in_block_lenght;
                }
            },
            Orientation::Right => {
                self.x += lenght as u64;
                if self.x2 + in_block_lenght >= 40  {
                    self.x += 1;
                }
                self.x2 = (self.x2 + in_block_lenght) % 40;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Entity;
    use crate::block::Orientation;
	
	#[test]
	fn moves() {
        let mut player = Entity::spawn_player(String::new());

        assert_eq!(player.get_coords(), (9_223_372_036_854_775_808, 9_223_372_036_854_775_808));
        assert_eq!(player.get_position_in_block(), (0, 0));

        for _ in 0..20 {
            player.move_in_direction(Orientation::Left);
        }
        
        assert_eq!(player.get_coords(), (9_223_372_036_854_775_807, 9_223_372_036_854_775_808));
        assert_eq!(player.get_position_in_block(), (20, 0));

        for _ in 0..30 {
            player.move_in_direction(Orientation::Left);
        }
        
        assert_eq!(player.get_coords(), (9_223_372_036_854_775_806, 9_223_372_036_854_775_808));
        assert_eq!(player.get_position_in_block(), (30, 0));

        for _ in 0..30 {
            player.move_in_direction(Orientation::Left);
        }

        assert_eq!(player.get_coords(), (9_223_372_036_854_775_806, 9_223_372_036_854_775_808));
        assert_eq!(player.get_position_in_block(), (0, 0));

        for _ in 0..30 {
            player.move_in_direction(Orientation::Down);
        }

        assert_eq!(player.get_coords(), (9_223_372_036_854_775_806, 9_223_372_036_854_775_808));
        assert_eq!(player.get_position_in_block(), (0, 30));

        for _ in 0..40 {
            player.move_in_direction(Orientation::Down);
        }

        assert_eq!(player.get_coords(), (9_223_372_036_854_775_806, 9_223_372_036_854_775_809));
        assert_eq!(player.get_position_in_block(), (0, 30));
	}
}
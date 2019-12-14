use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::convert::Into;
use serde::{Serialize, Deserialize};

/// A simple struct used to manage to axis coordinates
/// Sub and Add traits are implemented
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct Coords {
    pub x: SingleAxis,
    pub y: SingleAxis
}

impl Coords {
    pub fn new(x: SingleAxis, y: SingleAxis) -> Self {
        Self {
            x,
            y
        }
    }
}

impl Into<(u64, u64)> for Coords {
    fn into(self) -> (u64, u64) {
        (self.x.main, self.y.main)
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Coords {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl SubAssign for Coords {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Default for Coords {
    fn default() -> Self {
        Coords::new(SingleAxis::default(), SingleAxis::default())
    }
}

/// A simple struct used to manage a single axis
/// Use the main coordinates to store the coordinates
/// Use the additionnal coordinates to store where the player is located on the block located on the main coordinates
/// Additionnal value must not be higher than 127
/// You can modify directly the coordinates
#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SingleAxis {
    pub main: u64,
    additionnal: u8,
}

impl SingleAxis {
    pub fn new(main: u64, additionnal: u8) -> Self {
        assert!(additionnal < 127);
        SingleAxis {
            main,
            additionnal
        }
    }

    pub fn get_additionnal(&self) -> u8 {
        self.additionnal
    }
}

impl Default for SingleAxis {
    fn default() -> Self {
        SingleAxis::new(9_223_372_036_854_775_808, 0) // the center
    }
}

impl Add for SingleAxis {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut total_main = self.main + other.main;
        let total_additionnal = (self.additionnal + other.additionnal)%40;
        total_main += ((self.additionnal + other.additionnal) - total_additionnal) as u64 / 40;
        SingleAxis::new(total_main, total_additionnal)
    }
}

impl AddAssign for SingleAxis {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for SingleAxis {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut total_main = self.main - other.main;
        let mut total_additionnal = self.additionnal as i8 - other.additionnal as i8;
        while total_additionnal < 0 {
            total_additionnal += 40;
            total_main -= 1;
        }
        SingleAxis::new(total_main, total_additionnal as u8)
    }
}

impl SubAssign for SingleAxis {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let first = SingleAxis::default();
        let second = first + SingleAxis::new(1, 0);
        let third = second + SingleAxis::new(0, 40);
        let fourth = third + SingleAxis::new(1, 41);
        let fifth = fourth + SingleAxis::new(0, 38);
        let sixth = fifth + SingleAxis::new(0, 4);

        assert_eq!(first, SingleAxis::new(0, 0));
        assert_eq!(second, SingleAxis::new(1, 0));
        assert_eq!(third, SingleAxis::new(2, 0));
        assert_eq!(fourth, SingleAxis::new(4, 1));
        assert_eq!(fifth, SingleAxis::new(4, 39));
        assert_eq!(sixth, SingleAxis::new(5, 3));
    }

    #[test]
    fn sub_test() {
        let sixth = SingleAxis::new(5, 3);
        let fifth = sixth - SingleAxis::new(0, 4);
        let fourth = fifth - SingleAxis::new(0, 38);
        let third = fourth - SingleAxis::new(1, 41);
        let second = third - SingleAxis::new(0, 40);
        let first = second - SingleAxis::new(1, 0);

        assert_eq!(sixth, SingleAxis::new(5, 3));
        assert_eq!(fifth, SingleAxis::new(4, 39));
        assert_eq!(fourth, SingleAxis::new(4, 1));
        assert_eq!(third, SingleAxis::new(2, 0));
        assert_eq!(second, SingleAxis::new(1, 0));
        assert_eq!(first, SingleAxis::new(0, 0));
    }
}
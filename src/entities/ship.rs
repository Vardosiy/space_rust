use super::shape::Shape;
use super::shape::Shaped;

use std::time::{Duration, Instant};

pub struct Ship {
    shape: Shape,

    hp: i32,
    last_time_hit: Instant,
}

impl Shaped for Ship {
    fn shape(&self) -> &Shape {
        &self.shape
    }
    fn shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }
}

impl Ship {
    pub fn new(shape: Shape, hp: i32) -> Ship {
        Ship {
            shape,
            hp,
            last_time_hit: Instant::now(),
        }
    }

    pub fn hit(&mut self, damage: i32) {
        self.hp -= damage;
    }
    pub fn alive(&self) -> bool {
        self.hp > 0
    }
}

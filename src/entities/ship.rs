use super::destroyable::Destroyable;
use super::shape::Shape;
use super::shape::Shaped;

use std::time::{Duration, Instant};

pub struct Ship {
    shape: Shape,

    hp: i32,
    hp_max: i32,
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

impl Destroyable for Ship {
    fn hp(&self) -> i32 {
        self.hp
    }
    fn hp_mut(&mut self) -> &mut i32 {
        &mut self.hp
    }
    fn hp_max(&self) -> i32 {
        self.hp_max
    }
}

impl Ship {
    pub fn new(shape: Shape, hp: i32) -> Ship {
        Ship {
            shape,
            hp,
            hp_max: hp,
            last_time_hit: Instant::now(),
        }
    }
}

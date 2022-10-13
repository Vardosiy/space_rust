pub mod boss_stages;
pub mod easy_stages;
pub mod stage_factories;

use boss_stages::{BossStage, BossStagesFactory};

use super::shape::{Shape, Shaped};
use super::ship::Ship;
use super::shot::Shot;
use super::destroyable::Destroyable;

pub struct Boss {
    shape: Shape,

    stage_factory: Box<dyn BossStagesFactory>,
    stage: Box<dyn BossStage>,
    stage_idx: i32,

    hp: i32,
    max_hp: i32,
}

impl Shaped for Boss {
    fn shape(&self) -> &Shape {
        &self.shape
    }
    fn shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }
}

impl Destroyable for Boss {
    fn hp(&self) -> i32 {
        self.hp
    }
    fn hp_mut(&mut self) -> &mut i32 {
        &mut self.hp
    }
    fn hp_max(&self) -> i32 {
        self.max_hp
    }
}

impl Boss {
    pub fn new(shape: Shape, max_hp: i32, stage_factory: Box<dyn BossStagesFactory>) -> Boss {
        let stage_idx = 0;
        let initial_stage = stage_factory.create(stage_idx);
        Boss{
            shape,
            stage_factory,
            stage: initial_stage,
            stage_idx,
            hp: max_hp,
            max_hp
        }
    }

    pub fn fly(&mut self, ship: &Ship) {
        self.stage.update_pos(&mut self.shape, &ship)
    }

    pub fn shoot(&mut self, ship: &Ship) -> Option<Vec<Shot>> {
        self.stage.shoot(&self.shape, &ship)
    }
}

pub mod boss_stages;
pub mod easy_stages;
pub mod stage_factories;

use boss_stages::{BossStage, BossStagesFactory};

use super::shape::{Shape, Shaped};
use super::ship::Ship;
use super::shot::Shot;

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
        &shape
    }
    fn shape_mut(&mut self) -> &mut Shape {
        &mut shape
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

    pub fn hp(&self) -> i32 {
        self.hp
    }
    pub fn max_hp(&self) -> i32 {
        self.max_hp
    }

    pub fn hit(&mut self, damage: i32) {
        self.hp -= damage;
    }
    pub fn alive(&self) -> bool {
        self.hp > 0
    }

    pub fn update_pos(&mut self, ship: &Ship) {
        let new_pos = self.stage.calc_new_pos(&self, &ship);
        self.set_pos(new_pos);
    }
    pub fn shoot(&mut self, ship: &Ship) -> Option<Vec<Shot>> {
        self.stage.shoot(&self, &ship)
    }
}

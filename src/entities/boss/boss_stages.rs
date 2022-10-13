use crate::entities::shape::Shape;
use crate::entities::boss::Boss;
use crate::entities::ship::Ship;
use crate::entities::shot::Shot;

pub trait BossStage {
    fn update_pos(&mut self, boss_shape: &mut Shape, ship: &Ship);
    fn shoot(&mut self, boss_shape: &Shape, ship: &Ship) -> Option<Vec<Shot>>;

    fn completed(&self, boss: &Boss) -> bool;
}

pub trait BossStagesFactory {
    fn create(&self, idx: i32) -> Box<dyn BossStage>;
}

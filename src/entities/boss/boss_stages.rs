use crate::entities::boss::Boss;
use crate::entities::ship::Ship;
use crate::entities::shot::Shot;
use crate::math::Vec2i;

pub trait BossStage {
    fn calc_new_pos(&mut self, boss: &Boss, ship: &Ship) -> Vec2i;
    fn shoot(&mut self, boss: &Boss, ship: &Ship) -> Option<Vec<Shot>>;

    fn completed(&self, boss: &Boss) -> bool;
}

pub trait BossStagesFactory {
    fn create(&self, idx: i32) -> Box<dyn BossStage>;
}

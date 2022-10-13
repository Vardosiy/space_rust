use super::BossStage;
use super::BossStagesFactory;

use super::easy_stages::*;

pub struct EasyBossStageFactory;
impl BossStagesFactory for EasyBossStageFactory {
    fn create(&self, idx: i32) -> Box<dyn BossStage> {
        match idx {
            0 => Box::new(AppearStage::new()),
            1 => Box::new(SimpleShootingDown::new()),
            2 => Box::new(SpreadShooting::new()),
            3 => Box::new(Targeted::new()),
            _ => panic!("Index out of range")
        }
    }
}

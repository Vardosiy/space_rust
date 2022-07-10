use super::BossStage;
use super::BossStagesFactory;

use super::easy_stages::*;

pub struct EasyBossStageFactory;
impl StageFactory for EasyBossStageFactory {
    fn create(&self, idx: i32) -> Box<dyn BossStage> {
        let stage: dyn BossStage = match idx {
            0 => AppearStage::new(),
            1 => SimpleShootingDown::new(),
            2 => SpreadShooting::new(),
            3 => Targeted::new(),
            _ => panic!("Index out of range")
        };

        Box::new(stage)
    }
}
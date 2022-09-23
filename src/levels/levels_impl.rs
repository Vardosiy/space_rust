use crate::entities::comet::{Comet, CometKind};
use crate::entities::shape::{Shape, Shaped};
use crate::entities::boss::Boss;
use crate::entities::boss::stage_factories::*;
use crate::math::Vec2i;

use crate::globals::screen_size;

use super::level::Level;

use rand::Rng;

//-----------------------------------------------------------------------------

const EASY_LEVEL_BOSS_SPAWN_POINTS: i32 = 50;
const MEDIUM_LEVEL_BOSS_SPAWN_POINTS: i32 = 50;
const FREE_LEVEL_BOSS_SPAWN_POINTS: i32 = i32::MAX;

const EASY_LEVEL_BOSS_WIDTH: i32 = 100;
const EASY_LEVEL_BOSS_MAX_HP: i32 = 100;

//-----------------------------------------------------------------------------

fn spawn_comet_common(kind: CometKind, min_speed: i32) -> Comet {
    let screen_size = screen_size();

    let mut rng = rand::thread_rng();
    let spawn_x = rng.gen_range(0..screen_size.x);

    let angle_range = if spawn_x > screen_size.x / 2 { 180..270 } else { 270..360 };
    let angle = rng.gen_range(angle_range);

    let spawn_pos = Vec2i{ x: spawn_x, y: -Comet::get_width(kind) };
    Comet::new(kind, spawn_pos, angle, min_speed)
}

//-----------------------------------------------------------------------------

struct LevelEasy;
impl Level for LevelEasy {
    fn boss_spawn_points() -> i32 {
        EASY_LEVEL_BOSS_SPAWN_POINTS
    }

    fn spawn_comet(min_speed: i32) -> Comet {
        spawn_comet_common(CometKind::Simple, min_speed)
    }

    fn spawn_boss() -> Boss {
        let spawn_x = screen_size().x / 2 + EASY_LEVEL_BOSS_WIDTH / 2;
        let shape = Shape {
            pos: Vec2i { x: spawn_x, y: -EASY_LEVEL_BOSS_WIDTH },
            width: EASY_LEVEL_BOSS_WIDTH,
        };
        Boss::new(shape, EASY_LEVEL_BOSS_MAX_HP, Box::new(EasyBossStageFactory{}))
    }
}

struct LevelMedium;
impl Level for LevelMedium {
    fn boss_spawn_points() -> i32 {
        MEDIUM_LEVEL_BOSS_SPAWN_POINTS
    }

    fn spawn_comet(min_speed: i32) -> Comet {
        let kind = rand::random::<CometKind>();
        spawn_comet_common(kind, min_speed)
    }

    fn spawn_boss() -> Boss {
        todo!()
    }
}

struct FreeLevel;
impl Level for FreeLevel {
    fn boss_spawn_points() -> i32 {
        FREE_LEVEL_BOSS_SPAWN_POINTS
    }

    fn spawn_comet(min_speed: i32) -> Comet {
        let kind = rand::random::<CometKind>();
        spawn_comet_common(kind, min_speed)
    }

    fn spawn_boss() -> Boss {
        panic!("This should never happen")
    }
}

//-----------------------------------------------------------------------------

use crate::entities::comet::{Comet, CometKind};
use crate::entities::shape::Shape;
use crate::entities::boss::Boss;
use crate::entities::boss::stage_factories::*;
use crate::math::Vec2i;

use crate::globals::screen_size;
use crate::constants::*;

use rand::Rng;

const EASY_LEVEL_BOSS_SPAWN_POINTS: i32 = 50;
const MEDIUM_LEVEL_BOSS_SPAWN_POINTS: i32 = 50;
const FREE_LEVEL_BOSS_SPAWN_POINTS: i32 = i32::MAX;

const

//-----------------------------------------------------------------------------

fn get_comet_width(kind: CometKind) -> i32 {
    match kind {
        CometKind::Simple => SIMPLE_COMET_WIDTH,
        CometKind::Double => DOUBLE_COMET_WIDTH
    }
}

fn spawn_comet_common(kind: CometKind, min_speed: i32) -> Comet {
    let screen_size = screen_size();

    let mut rng = rand::thread_rng();
    let spawn_x = rng.gen_range(0..screen_size.x);

    let angle_range = if spawn_x > screen_size.x / 2 { 180..270 } else { 270..360 };
    let angle = rng.gen_range(angle_range);

    let width = get_comet_width(kind);
    let shape = Shape::new( Vec2i{ x: spawn_x, y: 0 - width }, width );
    Comet::new(kind, shape, angle,min_speed)
}

//-----------------------------------------------------------------------------

trait Level {
    fn boss_spawn_points() -> i32;

    fn spawn_comet(min_speed: i32) -> Comet;
    fn spawn_boss() -> Boss {
        let shape = Shape{ pos: Vec2i {}, width: 100 };
        let max_hp = 100;
        let stage_factory = Box::new(EasyBossStageFactory{});
        Boss::new(shape, max_hp, stage_factory)
    }

    fn calc_comets_limit(player_points: i32) -> i32 {
        player_points / POINTS_TO_ADD_COMET + MIN_COMETS
    }
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
}

//-----------------------------------------------------------------------------

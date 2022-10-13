use std::time::{Duration, Instant};

use crate::entities::shape::{Shape, Shaped};
use crate::entities::destroyable::Destroyable;
use crate::entities::ship::Ship;
use crate::entities::shot::Shot;

use crate::globals::screen_rect;
use crate::constants::{SHOT_SPEED, SHOT_WIDTH};

use super::boss_stages::BossStage;
use super::Boss;

//-----------------------------------------------------------------------------

const APPEAR_MOVE_SPEED: i32 = 8;
const APPEAR_TARGET_HEIGHT: i32 = 50;

const SIMPLE_SHOOTING_STAGE_MOVE_SPEED: i32 = 12;
const SIMPLE_SHOOTING_STAGE_SHOOTING_INTERVAL: Duration = Duration::from_millis(300);

const SPREAD_SHOOTING_STAGE_MOVE_SPEED: i32 = 8;
const SPREAD_SHOOTING_STAGE_SHOOTING_INTERVAL: Duration = Duration::from_millis(500);
const SPREAD_SHOOTING_ANGLE_RANGE: i32 = 120;
const SPREAD_SHOOTING_ANGLE_STEP: usize = 20;

const TARGETED_STAGE_MOVE_SPEED: i32 = 15;
const TARGETED_STAGE_SHOOTING_INTERVAL: Duration = Duration::from_millis(500);

const STAGE_1_FINISH_HP_THRESHOLD: f32 = 0.7f32;
const STAGE_2_FINISH_HP_THRESHOLD: f32 = 0.4f32;

const BOSS_DAMAGE: i32 = 10;

const ANGLE_DOWN: i32 = 180;

//-----------------------------------------------------------------------------

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}

fn move_horizontally(direction: &mut Direction, boss_shape: &mut Shape, move_speed: i32) {
    let x_offset = match direction {
        Direction::Left => -move_speed,
        Direction::Right => move_speed,
    };

    let mut new_pos = boss_shape.pos();
    let screen_rect = screen_rect();

    new_pos.x += x_offset;
    boss_shape.set_pos(new_pos);
    if !boss_shape.in_rect(&screen_rect) {
        new_pos.x -= x_offset * 2;
        boss_shape.set_pos(new_pos);
        *direction = match direction {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
    }
}

//-----------------------------------------------------------------------------

fn shoot_down(shoot_time: &mut Instant, boss_shape: &Shape, shooting_interval: Duration) -> Option<Vec<Shot>> {
    let now = Instant::now();
    if *shoot_time + shooting_interval <= now {
        *shoot_time = now;

        let shot = make_boss_shot(&boss_shape, ANGLE_DOWN);
        return Some(vec![shot]);
    }

    None
}

fn make_boss_shot(boss_shape: &Shape, angle: i32) -> Shot {
    let shot_shape = Shape::new(boss_shape.center(), SHOT_WIDTH);
    Shot::new(shot_shape, SHOT_SPEED, angle, BOSS_DAMAGE)
}

//-----------------------------------------------------------------------------

pub struct AppearStage;

impl AppearStage {
    pub fn new() -> Self {
        Self{}
    }
}

impl BossStage for AppearStage {
    fn update_pos(&mut self, boss_shape: &mut Shape, ship: &Ship) {
        let mut new_pos = boss_shape.pos();
        new_pos.y = (new_pos.y + APPEAR_MOVE_SPEED).clamp(0, APPEAR_TARGET_HEIGHT);
        boss_shape.set_pos(new_pos)
    }

    fn shoot(&mut self, boss_shape: &Shape, ship: &Ship) -> Option<Vec<Shot>> {
        None
    }

    fn completed(&self, boss: &Boss) -> bool {
        boss.shape().pos().y > APPEAR_TARGET_HEIGHT
    }
}

//-----------------------------------------------------------------------------

pub struct SimpleShootingDown {
    direction: Direction,
    shoot_time: Instant,
}

impl SimpleShootingDown {
    pub fn new() -> Self {
        Self {
            direction: Direction::Right,
            shoot_time: Instant::now(),
        }
    }
}

impl BossStage for SimpleShootingDown {
    fn update_pos(&mut self, boss_shape: &mut Shape, ship: &Ship) {
        move_horizontally(&mut self.direction, boss_shape, SIMPLE_SHOOTING_STAGE_MOVE_SPEED)
    }

    fn shoot(&mut self, boss_shape: &Shape, ship: &Ship) -> Option<Vec<Shot>> {
        shoot_down(&mut self.shoot_time, &boss_shape, SIMPLE_SHOOTING_STAGE_SHOOTING_INTERVAL)
    }

    fn completed(&self, boss: &Boss) -> bool {
        boss.hp_percent() < STAGE_1_FINISH_HP_THRESHOLD
    }
}

//-----------------------------------------------------------------------------

pub struct SpreadShooting {
    direction: Direction,
    shoot_time: Instant,
}

impl SpreadShooting {
    pub fn new() -> Self {
        Self {
            direction: Direction::Right,
            shoot_time: Instant::now(),
        }
    }
}

impl BossStage for SpreadShooting {
    fn update_pos(&mut self, boss_shape: &mut Shape, ship: &Ship) {
        move_horizontally(&mut self.direction, boss_shape, SPREAD_SHOOTING_STAGE_MOVE_SPEED)
    }

    fn shoot(&mut self, boss_shape: &Shape, ship: &Ship) -> Option<Vec<Shot>> {
        let now = Instant::now();
        if self.shoot_time + SPREAD_SHOOTING_STAGE_SHOOTING_INTERVAL > now {
            return None;
        }

        self.shoot_time = now;

        let angle_start = ANGLE_DOWN - SPREAD_SHOOTING_ANGLE_RANGE / 2;
        let angle_end = ANGLE_DOWN + SPREAD_SHOOTING_ANGLE_RANGE / 2;

        let mut shots = vec![];
        for shot_angle in (angle_start..=angle_end).step_by(SPREAD_SHOOTING_ANGLE_STEP) {
            let shot = make_boss_shot(&boss_shape, shot_angle);
            shots.push(shot);
        }

        Some(shots)
    }

    fn completed(&self, boss: &Boss) -> bool {
        boss.hp_percent() < STAGE_2_FINISH_HP_THRESHOLD
    }
}

//-----------------------------------------------------------------------------

pub struct Targeted {
    shoot_time: Instant,
}

impl Targeted {
    pub fn new() -> Self {
        Self {
            shoot_time: Instant::now(),
        }
    }
}

impl BossStage for Targeted {
    fn update_pos(&mut self, boss_shape: &mut Shape, ship: &Ship) {
        let boss_center = boss_shape.center();
        let ship_center = ship.shape().center();
        let diff_x = boss_center.x - ship_center.x;

        let mut result = boss_shape.pos();
        if diff_x.abs() < TARGETED_STAGE_MOVE_SPEED {
            result.x = ship_center.x - boss_shape.width() / 2;
        } else {
            result.x += if diff_x > 0 {
                TARGETED_STAGE_MOVE_SPEED
            } else {
                -TARGETED_STAGE_MOVE_SPEED
            };
        }

        let screen_rect = screen_rect();
        result.x = result.x.clamp(screen_rect.top_left.x, screen_rect.bottom_right.x);

        boss_shape.set_pos(result)
    }

    fn shoot(&mut self, boss_shape: &Shape, ship: &Ship) -> Option<Vec<Shot>> {
        shoot_down(&mut self.shoot_time, &boss_shape, TARGETED_STAGE_SHOOTING_INTERVAL)
    }

    fn completed(&self, boss: &Boss) -> bool {
        false
    }
}

//-----------------------------------------------------------------------------

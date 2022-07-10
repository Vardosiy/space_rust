use crate::entities::shape::{Shape, Shaped};
use crate::entities::comet::Comet;
use crate::entities::ship::Ship;
use crate::entities::shot::Shot;

use crate::math::Vec2i;

use crate::globals::{screen_rect, screen_size};
use crate::constants::*;
use crate::entities::boss::Boss;

pub struct Game {
    comets: Vec<Comet>,

    ship: Ship,
    shots: Vec<Shot>,

    level: Box<dyn Level>,
    boss: Option<Boss>
}

impl Game {
    pub fn new() -> Game {
        let ship_shape = Shape::new(Vec2i { x: 0, y: 0 }, SHIP_WIDTH);
        Game {
            comets: vec![],
            shots: vec![],
            ship: Ship::new(ship_shape, SHIP_MAX_HP),
            boss: None
        }
    }

    pub fn start(&mut self) {
        self.ship = Ship::new(ship_shape, SHIP_MAX_HP);
    }

    pub fn stop(&mut self) {
        self.comets.clear();
        self.shots.clear();
    }

    pub fn update(&mut self) {
        self.handle_intersections();
        if !self.ship.alive() {
            self.stop_game();
        }
        self.handle_objects_off_screen();

        self.move_entities();
        self.spawn_entities();
    }

    pub fn render(&self) {
        ()
    }

    fn handle_intersections(&mut self) {
        self.shots.retain(|shot| {
            let remove = self.destroy_comets_by_shot(shot);
            !remove
        });
        self.comets.retain(|comet| {
            let intersects = self.ship.intersects(comet);
            if intersects {
                self.ship.hit(SIMPLE_COMET_DAMAGE)
            }
            !intersects
        });
    }
    fn handle_objects_off_screen(&mut self) {
        let screen_rect = screen_rect();

        self.shots.retain(|shot| {
            shot.appear_in_rect(&screen_rect)
        });
        self.comets.retain(|comet| {
            comet.appear_in_rect(&screen_rect)
        });
    }

    fn destroy_comets_by_shot(&mut self, shot: &Shot) -> bool {
        let idx = self.comets.iter().position(|x| x.intersects(shot));
        if let Some(idx) = idx {
            let destroyed = self.comets.remove(idx);
            // add spawn of comet shards
            return true;
        }

        false
    }

    fn move_entities(&mut self) {
        self.comets.iter_mut().for_each(|comet| comet.fly());
        self.shots.iter_mut().for_each(|shot| shot.fly());
    }

    fn spawn_entities(&mut self) {
        let player_points = 10;

        if player_points >= self.level.boss_spawn_points() {
            self.boss = self.level.spawn_boss();
        }

        if self.comets.len() < level.calc_comets_limit(player_points) {
            self.comets.push(level.spawn_comet())
        }
    }
}

use crate::entities::shape::{Shape, Shaped};
use crate::entities::comet::Comet;
use crate::entities::ship::Ship;
use crate::entities::shot::Shot;
use crate::entities::boss::Boss;

use crate::levels::level::Level;
use crate::math::Vec2i;

use crate::globals::{screen_rect, screen_size};
use crate::constants::*;

pub struct Game {
    comets: Vec<Comet>,

    ship: Ship,
    ship_shots: Vec<Shot>,
    ship_controller: PlayerShipController,

    boss: Option<Boss>,
    boss_shots: Vec<Shot>,
    boss_defeated: bool,

    level: Box<dyn Level>,
}

impl Game {
    pub fn new() -> Game {
        let ship_shape = Shape::new(Vec2i { x: 0, y: 0 }, SHIP_WIDTH);
        Game {
            comets: vec![],
            ship_shots: vec![],
            ship: Ship::new(ship_shape, SHIP_MAX_HP),
            boss: None
        }
    }

    pub fn start(&mut self) {
        let ship_shape = Shape::new(Vec2i { x: 0, y: 0 }, SHIP_WIDTH);
        self.ship = Ship::new(ship_shape, SHIP_MAX_HP);
    }

    pub fn stop(&mut self) {
        self.comets.clear();
        self.ship_shots.clear();
    }

    pub fn render(&self) {
        ()
    }

    pub fn update(&mut self) {
        self.handle_intersections();
        if !self.ship.alive() {
            self.stop();
        }
        self.handle_objects_off_screen();

        self.move_entities();
        self.spawn_entities();
    }

    fn move_entities(&mut self) {
        self.comets.iter_mut().for_each(|comet| comet.fly());
        self.ship_shots.iter_mut().for_each(|shot| shot.fly());

        self.ship_controller.update(self.ship);

        if let Some(boss) = &self.boss {
            self.boss.fly();
        }
    }

    fn handle_intersections(&mut self) {
        self.comets.retain(|comet| {
            let intersects = self.ship.intersects(comet);
            if intersects {
                let comet_damage = comet.damage();
                self.ship.hit(comet_damage);
            }
            !intersects
        });
        self.boss_shots.retain(|boss_shot| {
            let intersects = self.ship.intersects(boss_shot);
            if intersects {
                self.ship.hit(boss_shot.damage());
            }
            !intersects
        });

        self.ship_shots.retain(|shot| {
            let remove = self.destroy_comets_by_shot(shot);
            !remove
        });

        if let Some(boss) = &self.boss {  // TODO: play with borriwng here
            self.ship_shots.retain(|shot| {
                let intersects = boss.intersects(shot);
                if intersects {
                    boss.hit(shot.damage())
                }
                !intersects
            });

            if !boss.alive() {
                self.boss = None;
                self.boss_defeated = true;
                // TODO destroy animation
            }
        }
    }
    fn handle_objects_off_screen(&mut self) {
        let screen_rect = screen_rect();

        self.ship_shots.retain(|shot| {
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
            if let Some(shards_from_destoryed) = destroyed.spawn_shards() {
                self.comets.append(&mut shards_from_destoryed);
            }
            return true;
        }

        false
    }

    fn spawn_entities(&mut self) {
        let player_points = 10;  // TODO implement player_points

        if player_points >= self.level.boss_spawn_points() {
            self.boss = self.level.spawn_boss();
        }

        if self.boss.is_none() {
            let comet_limit = self.level.calc_comets_limit(player_points);
            if self.comets.len() < comet_limit {
                self.comets.push(self.level.spawn_comet())
            }
        }
    }
}

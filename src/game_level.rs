use std::rc::Rc;

use crate::entities::shape::{Shape, Shaped};
use crate::entities::destroyable::Destroyable;
use crate::entities::comet::Comet;
use crate::entities::ship::Ship;
use crate::entities::shot::Shot;
use crate::entities::boss::Boss;

use crate::player_ship_controller::PlayerShipController;
use crate::spawners::Spawner;
use crate::spawners::spawners_impl::*;

use crate::input_mgr::InputMgr;
use crate::globals::screen_rect;

use crate::math::Vec2i;
use crate::constants::*;

pub struct GameLevel {
    comets: Vec<Comet>,

    ship: Ship,
    ship_shots: Vec<Shot>,
    ship_controller: PlayerShipController,

    boss: Option<Boss>,
    boss_shots: Vec<Shot>,
    boss_defeated: bool,

    spawner: Box<dyn Spawner>,
}

impl GameLevel {
    pub fn new(input_mgr: Rc<InputMgr>) -> GameLevel {
        let ship_shape = Shape::new(Vec2i { x: 0, y: 0 }, SHIP_WIDTH);
        let ship = Ship::new(ship_shape, SHIP_MAX_HP);

        let ship_controller = PlayerShipController::new(Rc::clone(&input_mgr));
        let spawner = Box::new(EasyLevelSpawner{});
        GameLevel {
            comets: vec![],
            ship_shots: vec![],
            ship,
            ship_controller,

            boss: None,
            boss_shots: vec![],
            boss_defeated: false,

            spawner,
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

        self.ship_controller.update(&mut self.ship);

        if let Some(boss) = &mut self.boss {
            boss.fly(&self.ship);
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

        let comets = &mut self.comets;
        self.ship_shots.retain(|shot| {
            let remove = GameLevel::destroy_comets_by_shot(comets, shot);
            !remove
        });

        if let Some(boss) = &mut self.boss {  // TODO: play with borriwng here
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
            shot.shape().appear_in_rect(&screen_rect)
        });
        self.comets.retain(|comet| {
            comet.shape().appear_in_rect(&screen_rect)
        });
    }

    fn destroy_comets_by_shot(comets: &mut Vec<Comet>, shot: &Shot) -> bool {
        let idx = comets.iter().position(|x| x.intersects(shot));
        if let Some(idx) = idx {
            let destroyed = comets.remove(idx);
            if let Some(mut shards_from_destroyed) = destroyed.spawn_shards() {
                comets.append(&mut shards_from_destroyed);
            }
            return true;
        }

        false
    }

    fn spawn_entities(&mut self) {
        let player_points = 10;  // TODO implement player_points

        if player_points >= self.spawner.boss_spawn_points() {
            self.boss = Some(self.spawner.spawn_boss());
        }

        if self.boss.is_none() {
            let comets_limit = self.spawner.calc_comets_limit(player_points);
            if self.comets.len() < comets_limit as usize {
                self.comets.push(self.spawner.spawn_comet(3))
            }
        }
    }
}

pub mod spawners_impl;

use crate::constants::*;
use crate::entities::boss::Boss;
use crate::entities::comet::Comet;

pub trait Spawner {
    fn boss_spawn_points(&self) -> i32;

    fn spawn_comet(&self, min_speed: i32) -> Comet;
    fn spawn_boss(&self) -> Boss;

    fn calc_comets_limit(&self, player_points: i32) -> i32 {
        player_points / POINTS_TO_ADD_COMET + MIN_COMETS
    }
}

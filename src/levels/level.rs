use crate::constants::*;
use crate::entities::boss::Boss;
use crate::entities::comet::Comet;

pub trait Level {
    fn boss_spawn_points() -> i32;

    fn spawn_comet(min_speed: i32) -> Comet;
    fn spawn_boss() -> Boss;

    fn calc_comets_limit(player_points: i32) -> i32 {
        player_points / POINTS_TO_ADD_COMET + MIN_COMETS
    }
}

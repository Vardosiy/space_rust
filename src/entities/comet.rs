use rand::Rng;

use super::shape::{Shape, Shaped};

use crate::constants::*;
use crate::math::Vec2i;

#[derive(Copy, Clone)]
pub enum CometKind {
    Simple,
    Double,
}

pub struct Comet {
    kind: CometKind,
    shape: Shape,

    angle: i32,
    speed: i32,
}

impl Shaped for Comet {
    fn shape(&self) -> &Shape {
        &self.shape
    }
    fn shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }
}

impl Comet {
    pub fn new(kind: CometKind, pos: Vec2i, angle: i32, speed: i32) -> Comet {
        let width = Comet::get_width(kind);
        let shape = Shape::new(pos, width);
        Comet {
            kind,
            shape,
            angle,
            speed,
        }
    }

    pub fn get_width(kind: CometKind) -> i32 {
        match kind {
            CometKind::Simple => SIMPLE_COMET_WIDTH,
            CometKind::Double => DOUBLE_COMET_WIDTH,
        }
    }

    pub fn damage(&self) -> i32 {
        match self.kind {
            CometKind::Simple => SIMPLE_COMET_DAMAGE,
            CometKind::Double => DOUBLE_COMET_DAMAGE,
        }
    }

    pub fn kind(&self) -> CometKind {
        self.kind
    }

    pub fn fly(&mut self) {
        let angle_rad = (self.angle as f32).to_radians();

        let mut pos = self.pos();
        pos.x += self.speed * angle_rad.sin();
        pos.y -= self.speed * angle_rad.cos();
        self.set_pos(pos);
    }

    pub fn spawn_shards(&self) -> Option<Vec<Comet>> {
        match self.kind {
            CometKind::Simple => None,
            CometKind::Double => {
                let mut rng = rand::thread_rng();
                let shard1_angle_delta = rng.gen_range(0..COMET_SHARD_MAX_ANGLE_DELTA);
                let shard2_angle_delta = rng.gen_range(0..-COMET_SHARD_MAX_ANGLE_DELTA);

                let mut result;
                for angle_delta in (shard1_angle_delta, shard2_angle_delta) {
                    let shardKind = CometKind::Simple;
                    let shard = Comet::new(shardKind, self.pos, self.angle, self.speed);
                    result.append(shard);
                }

                Some(result)
            }
        }
    }
}

use rand::Rng;
use rand::distributions::{Distribution, Standard};

use super::shape::{Shape, Shaped};

use crate::constants::*;
use crate::math::Vec2i;

#[derive(Copy, Clone)]
pub enum CometKind {
    Simple,
    Double,
}

impl Distribution<CometKind> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CometKind {
        match rng.gen_range(0..1) {
            0 => CometKind::Simple,
            1 => CometKind::Double,
            _ => CometKind::Simple
        }
    }
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
        let x_diff = (self.speed as f32) * angle_rad.sin();
        let y_diff =  (self.speed as f32) * angle_rad.cos();

        let mut pos = self.shape.pos();
        pos.x += x_diff as i32;
        pos.y -= y_diff as i32;
        self.shape.set_pos(pos);
    }

    pub fn spawn_shards(&self) -> Option<Vec<Comet>> {
        match self.kind {
            CometKind::Simple => None,
            CometKind::Double => {
                let mut rng = rand::thread_rng();
                let shard1_angle_delta = rng.gen_range(0..COMET_SHARD_MAX_ANGLE_DELTA);
                let shard2_angle_delta = rng.gen_range(0..-COMET_SHARD_MAX_ANGLE_DELTA);

                let mut result = vec![];
                for angle_delta in [shard1_angle_delta, -shard2_angle_delta] {
                    let shard_kind = CometKind::Simple;
                    let shard = Comet::new(shard_kind, self.shape.pos(), self.angle + angle_delta, self.speed);
                    result.push(shard);
                }

                Some(result)
            }
        }
    }
}

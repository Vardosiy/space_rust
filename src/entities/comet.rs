use super::shape::Shape;
use super::shape::Shaped;

#[derive(Copy, Clone)]
pub enum CometKind {
    Simple,
    Double
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
    pub fn new(kind: CometKind, shape: Shape, angle: i32, speed: i32) -> Comet {
        Comet{
            kind,
            shape,
            angle,
            speed
        }
    }

    pub fn fly(&mut self) {
        let angle_rad = (self.angle as f32).to_radians();

        let mut pos = self.pos();
        pos.x += self.speed * angle_rad.sin();
        pos.y -= self.speed * angle_rad.cos();
        self.set_pos(pos);
    }
}

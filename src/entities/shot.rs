use super::shape::Shape;
use super::shape::Shaped;

pub struct Shot {
    shape: Shape,

    speed: i32,
    angle: i32,

    damage: i32,
}

impl Shaped for Shot {
    fn shape(&self) -> &Shape {
        &self.shape
    }
    fn shape_mut(&mut self) -> &mut Shape {
        &mut self.shape
    }
}

impl Shot {
    pub fn new(shape: Shape, speed: i32, angle: i32, damage: i32) -> Shot {
        Shot {
            shape,
            speed,
            angle,
            damage,
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

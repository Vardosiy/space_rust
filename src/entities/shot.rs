use super::shape::{Shape, Shaped};

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
        let x_diff = (self.speed as f32) * angle_rad.sin();
        let y_diff =  (self.speed as f32) * angle_rad.cos();

        let mut pos = self.shape.pos();
        pos.x += x_diff as i32;
        pos.y -= y_diff as i32;
        self.shape.set_pos(pos);
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }
}

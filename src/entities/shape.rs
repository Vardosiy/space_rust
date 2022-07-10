use crate::math::Vec2i;
use crate::math::Rect;

#[derive(Clone)]
pub struct Shape {
    pub pos: Vec2i,
    pub width: i32,
}

impl Shape {
    pub fn new(pos: Vec2i, width: i32) -> Shape {
        Shape { pos, width }
    }

    pub fn in_rect(&self, rect: &Rect) -> bool {
        let pos = &self.pos;

        let x_in_rect = pos.x >= rect.top_left.x && pos.x <= rect.bottom_right.x - self.width;
        let y_in_rect = pos.y >= rect.top_left.y && pos.y <= rect.bottom_right.y - self.width;

        x_in_rect && y_in_rect
    }
    pub fn appear_in_rect(&self, rect: &Rect) -> bool {
        let appear_bound = Rect{
            top_left: rect.top_left - self.width,
            bottom_right: rect.bottom_right + self.width
        };

        self.in_rect(&appear_bound)
    }
}

pub trait Shaped {
    fn shape(&self) -> &Shape;
    fn shape_mut(&mut self) -> &mut Shape;

    fn pos(&self) -> Vec2i {
        self.shape().pos
    }
    fn set_pos(&mut self, pos: Vec2i) {
        self.shape_mut().set_pos(pos)
    }

    fn width(&self) -> i32 {
        self.shape().width
    }
    fn center(&self) -> Vec2i {
        self.pos() + self.width() / 2
    }

    fn in_rect(&self, rect: &Rect) -> bool {
        self.shape().in_rect(rect)
    }
    fn appear_in_rect(&self, rect: &Rect) -> bool {
        self.shape().appear_in_rect(rect)
    }

    fn intersects<T: Shaped>(&self, rhs: &T) -> bool {
        // TODO remove template
        let pos_diff = self.center() - rhs.center();
        let distance_square = pos_diff.x.pow(2) + pos_diff.y.pow(2);
        let intersection_distance_square = (self.width() / 2 + rhs.width() / 2).pow(2);

        return distance_square > intersection_distance_square;
    }
}

use crate::math::Rect;
use crate::math::Vec2i;

//-----------------------------------------------------------------------------

#[derive(Clone)]
pub struct Shape {
    pos: Vec2i,
    width: i32,
}

//-----------------------------------------------------------------------------

impl Shape {
    pub fn new(pos: Vec2i, width: i32) -> Shape {
        Shape { pos, width }
    }

    pub fn pos(&self) -> Vec2i {
        self.pos
    }
    pub fn set_pos(&mut self, pos: Vec2i) {
        self.pos = pos
    }

    pub fn width(&self) -> i32 {
        self.width
    }
    pub fn center(&self) -> Vec2i {
        self.pos() + self.width() / 2
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

//-----------------------------------------------------------------------------

pub trait Shaped {
    fn shape(&self) -> &Shape;
    fn shape_mut(&mut self) -> &mut Shape;

    fn intersects<T: Shaped>(&self, rhs: &T) -> bool {
        let lhs_shape = self.shape();
        let rhs_shape = rhs.shape();

        let pos_diff = lhs_shape.center() - rhs_shape.center();
        let distance_square = pos_diff.x.pow(2) + pos_diff.y.pow(2);
        let intersection_distance_square = (lhs_shape.width() / 2 + rhs_shape.width() / 2).pow(2);

        return distance_square > intersection_distance_square;
    }
}

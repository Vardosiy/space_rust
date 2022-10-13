use crate::math::{Rect, Vec2i};

static mut G_SCREEN_SIZE: Vec2i = Vec2i { x: 0, y: 0};

pub fn set_screen_size(size: Vec2i) {
    unsafe {
        G_SCREEN_SIZE = size;
    }
}

pub fn screen_size() -> Vec2i {
    unsafe {
        return G_SCREEN_SIZE.clone();
    }
}

pub fn screen_rect() -> Rect {  // TODO change this to a common coords
    Rect{
        top_left: Vec2i { x: 0, y: 0 },
        bottom_right: screen_size()
    }
}

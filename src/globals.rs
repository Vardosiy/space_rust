use crate::math::{Rect, Vec2i};

static mut g_screen_size: Vec2i = Vec2i { x: 0, y: 0};

pub fn screen_size() -> Vec2i {
    unsafe {
        return g_screen_size.clone();
    }
}

pub fn screen_rect() -> Rect {  // TODO change this to a common coords
    Rect{
        top_left: Vec2i { x: 0, y: 0 },
        bottom_right: screen_size()
    }
}

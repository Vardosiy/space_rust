use std::rc::Rc;

use crate::constants::SHIP_MOVE_STEP;
use crate::entities::shape::Shaped;
use crate::entities::ship::Ship;
use crate::input_mgr::InputMgr;

pub struct PlayerShipController {
    input_mgr: Rc<InputMgr>
}

impl PlayerShipController {
    pub fn new(input_mgr: Rc<InputMgr>) -> Self {
        Self { input_mgr }
    }

    pub fn update(&self, ship: &mut Ship) {
        let shape = ship.shape_mut();
        let mut new_pos = shape.pos();
        new_pos.x -= SHIP_MOVE_STEP * self.input_mgr.is_pressed('A') as i32;
        new_pos.x += SHIP_MOVE_STEP * self.input_mgr.is_pressed('D') as i32;
        new_pos.y += SHIP_MOVE_STEP * self.input_mgr.is_pressed('W') as i32;
        new_pos.y -= SHIP_MOVE_STEP * self.input_mgr.is_pressed('S') as i32;
        shape.set_pos(new_pos);
    }
}

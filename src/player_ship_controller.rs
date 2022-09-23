use std::rc::Rc;

use crate::entities::{ship::Ship, shape::Shaped};

const SHIP_MOVE_STEP: i32 = 5; // TODO fix costant value

struct PlayerShipController {
    input_mgr: Rc<InputMgr>
}

impl PlayerShipController {
    pub fn new(input_mgr: Rc<InputMgr>) -> Self {
        Self { input_mgr }
    }

    pub fn update(&self, ship: &mut Ship) {
        let pos = ship.pos();

        if self.input_mgr.is_pressed('A') {
            pos.x -= SHIP_MOVE_STEP;
        }
        if self.input_mgr.is_pressed('D') {
            pos.x += SHIP_MOVE_STEP;
        }

        if self.input_mgr.is_pressed('W') {
            pos.y += SHIP_MOVE_STEP;
        }
        if self.input_mgr.is_pressed('S') {
            pos.y -= SHIP_MOVE_STEP;
        }

        ship.set_pos(pos);
    }
}

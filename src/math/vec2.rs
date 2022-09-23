#[derive(Copy, Clone)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

//-----------------------------------------------------------------------------

impl std::ops::Add<Vec2i> for Vec2i {
    type Output = Vec2i;

    fn add(self, rhs: Vec2i) -> Self::Output {
        Vec2i{ x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Add<i32> for Vec2i {
    type Output = Vec2i;

    fn add(self, scalar: i32) -> Self::Output {
        Vec2i{ x: self.x + scalar, y: self.y + scalar }
    }
}

//-----------------------------------------------------------------------------

impl std::ops::Sub<Vec2i> for Vec2i {
    type Output = Vec2i;

    fn sub(self, rhs: Vec2i) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Sub<i32> for Vec2i {
    type Output = Vec2i;

    fn sub(self, scalar: i32) -> Self::Output {
        Self::Output {
            x: self.x - scalar,
            y: self.y - scalar,
        }
    }
}

//-----------------------------------------------------------------------------

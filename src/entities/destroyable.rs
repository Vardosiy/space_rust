pub trait Destroyable {
    fn hp(&self) -> i32;
    fn hp_mut(&mut self) -> &mut i32;
    fn hp_max(&self) -> i32;

    fn hp_percent(&self) -> f32 {
        let result = self.hp() as f32 / self.hp_max() as f32;
        result.max(0.0f32)
    }

    fn hit(&mut self, damage: i32) {
        *self.hp_mut() -= damage;
    }
    fn alive(&self) -> bool {
        self.hp() > 0
    }
}

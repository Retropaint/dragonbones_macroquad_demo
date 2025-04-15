#[derive(Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Default)]
pub struct Model {
    pub pos: Vec2,
    pub rot: Vec2,
    pub scale: Vec2,
}

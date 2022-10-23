use nannou::prelude::Vec2;

use crate::world::SIZE_MAP;

pub struct Settings {
    pub scale: f32,
    pub position: Vec2,
    pub mouse: Mouse,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            scale: 1.0,
            position: Vec2::new(SIZE_MAP.0 as f32, SIZE_MAP.1 as f32),
            mouse: Mouse::new(),
        }
    }
}

pub struct Mouse {
    pub pressed: bool,
    pub last_pos: (f64, f64),

}

impl Mouse {
    pub fn new() -> Self {
        Self {
            pressed: false,
            last_pos: (0.0, 0.0)
        }
    }
}
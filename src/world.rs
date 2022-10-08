use crate::cell::Cell;

pub const SIZE_MAP: (usize, usize) = (40, 40);

pub struct World {
    pub cells: ([[u8; SIZE_MAP.0]; SIZE_MAP.1], Vec<Cell>),
    pub nutrient_medium: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            cells: ([[0; SIZE_MAP.0]; SIZE_MAP.1], vec![Cell::new((25, 0))]),
            nutrient_medium: 1.2
        }
    }
}
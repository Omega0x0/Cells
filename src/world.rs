use crate::cell::Cell;

pub const SIZE_MAP: (usize, usize) = (50, 50);

pub struct World {
    pub cells: ([[u8; SIZE_MAP.0]; SIZE_MAP.1], Vec<Cell>)
}

impl World {
    pub fn new() -> Self {
        Self {
            cells: ([[0; SIZE_MAP.0]; SIZE_MAP.1], vec![Cell::new((25, 25))])
        }
    }
}
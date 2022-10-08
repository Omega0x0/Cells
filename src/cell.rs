use crate::color::ColorCell;

#[derive(Clone, Debug)]
pub struct Cell {
    pub position: (usize, usize),
    pub color: ColorCell,
    pub direction: i8,
    pub step: usize,
    pub genome: Vec<Gen>
}

impl Cell {
    pub fn new(pos: (usize, usize)) -> Self {
        Self {
            position: pos,
            color: ColorCell::new(1., 1., 1.),
            direction: 0,
            step: 0,
            genome: vec![
                Gen::SetDirection(1),
                Gen::Reproduce,
            ]
        }
    }

    pub fn to_rotate(&mut self, direction: i8) {
        self.direction += direction;

        if self.direction > 3 { self.direction = 0; }
        else if self.direction < 0 { self.direction = 3; }
    }
}

#[derive(Clone, Debug)]
pub enum Gen {
    SetDirection(i8),
    Reproduce,
}
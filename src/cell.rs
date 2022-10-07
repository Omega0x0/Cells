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
}

#[derive(Clone, Debug)]
pub enum Gen {
    SetDirection(i8),
    Reproduce,
}
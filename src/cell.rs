use crate::color::ColorCell;

pub const DEFAULT_MAX_TIME_LIFE: usize = 13;
pub const DEFAULT_MIN_MASS: f32 = 12.0;
pub const DEFAULT_MASS: f32 = 25.0;
pub const DEFAULT_MAX_MASS: f32 = 30.0;
pub const DEFAULT_MIN_MASS_DIVISION: f32 = 26.0;

#[derive(Clone, Debug)]
pub struct Cell {
    pub position: (usize, usize),
    pub color: ColorCell,
    pub direction: i8,

    pub time_life: usize,
    pub max_time_life: usize,
    pub min_mass: f32,
    pub mass: f32,
    pub max_mass: f32,
    pub min_mass_division: f32,

    pub step: usize,
    pub genome: Vec<Gen>
}

impl Cell {
    pub fn new(pos: (usize, usize)) -> Self {
        Self {
            position: pos,
            color: ColorCell::new(1., 1., 1.),
            direction: 0,

            time_life: 0,
            max_time_life: DEFAULT_MAX_TIME_LIFE,
            min_mass: DEFAULT_MIN_MASS,
            mass: DEFAULT_MASS,
            max_mass: DEFAULT_MAX_MASS,
            min_mass_division: DEFAULT_MIN_MASS_DIVISION,

            step: 0,
            genome: vec![
                Gen::SetDirection(1),
                Gen::Reproduce,
            ]
        }
    }

    pub fn to_rotate(&mut self, direction: i8) {
        self.direction = (self.direction + direction) % 4;
    }
}

#[derive(Clone, Debug)]
pub enum Gen {
    SetDirection(i8),
    Reproduce,
}
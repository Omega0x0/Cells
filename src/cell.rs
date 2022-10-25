use rand::Rng;

use crate::color::ColorCell;

pub const DEFAULT_MAX_TIME_LIFE: usize = 13;
pub const DEFAULT_MIN_MASS: f32 = 12.0;
pub const DEFAULT_MASS: f32 = 25.0;
pub const DEFAULT_MAX_MASS: f32 = 30.0;
pub const DEFAULT_MIN_MASS_DIVISION: f32 = 26.0;
pub const DEFAULT_DAMAGE: f32 = 1.0;
pub const DEFAULT_RESISTANCE: f32 = 1.0;

#[derive(Clone, Debug)]
pub struct Cell {
    pub species: usize,
    pub position: (usize, usize),
    pub color: ColorCell,
    pub direction: i8,

    pub time_life: usize,
    pub max_time_life: usize,
    pub min_mass: f32,
    pub mass: f32,
    pub max_mass: f32,
    pub min_mass_division: f32,
    pub damage: f32,
    pub resistance: f32,

    pub step: usize,
    pub genome: Vec<Gen>
}

impl Cell {
    pub fn new(pos: (usize, usize)) -> Self {
        Self {
            species: 0,
            position: pos,
            color: ColorCell::new(1., 1., 1.),
            direction: 0,

            time_life: 0,
            max_time_life: DEFAULT_MAX_TIME_LIFE,
            min_mass: DEFAULT_MIN_MASS,
            mass: DEFAULT_MASS,
            max_mass: DEFAULT_MAX_MASS,
            min_mass_division: DEFAULT_MIN_MASS_DIVISION,
            damage: DEFAULT_DAMAGE,
            resistance: DEFAULT_RESISTANCE,

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

    pub fn mutate(&mut self) {
        if rand::thread_rng().gen_range(0.0..1.0) < 0.01 {
            self.species = rand::thread_rng().gen_range(0..1_000_000_000);

            self.max_time_life = (self.max_time_life as i64 + rand::thread_rng().gen_range(-1..=1)) as usize;

            self.min_mass += rand::thread_rng().gen_range(-1.0..=1.0);
            self.min_mass_division += rand::thread_rng().gen_range(-1.0..=1.0);
            self.max_mass += rand::thread_rng().gen_range(-1.0..=1.0);
            self.damage += rand::thread_rng().gen_range(-1.0..=1.0);
            self.resistance += rand::thread_rng().gen_range(-1.0..=1.0);

            let rand_k = rand::thread_rng().gen_range(0..3);
            if rand_k == 0 {
                for gen in self.genome.iter_mut() {
                    match gen {
                        Gen::SetDirection(d) => {
                            *d += rand::thread_rng().gen_range(-1..=1);
                        },
                        _ => {}
                    }
                }
            } else if rand_k == 1 {
                let gen_i = rand::thread_rng().gen_range(0..3);
                match gen_i {
                    0 => self.genome.push(Gen::SetDirection(rand::thread_rng().gen_range(0..4))),
                    1 => self.genome.push(Gen::Reproduce),
                    2 => self.genome.push(Gen::Attack),
                    _ => {}
                }
                
                if self.genome.len() > 20 { self.mass = -1.0; }
            } else if rand_k == 2 {
                let gen_i = rand::thread_rng().gen_range(0..self.genome.len());
                
                if self.genome.len() > 1 {
                    self.genome.remove(gen_i);
                }

                if self.genome.len() == 0 
                || self.damage < 0.0 
                || self.resistance < 0.0 { 
                    self.mass = -1.0; 
                }
            }

            self.color.modify();
        }
    }

    pub fn consume(&mut self) -> f32 {
        return
            DEFAULT_MIN_MASS / self.min_mass +
            self.max_mass / DEFAULT_MAX_MASS +
            self.time_life as f32 / self.max_time_life as f32;
    }
}

#[derive(Clone, Debug)]
pub enum Gen {
    SetDirection(i8),
    Reproduce,
    Attack,
}
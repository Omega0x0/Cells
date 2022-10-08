pub struct Info {
    pub ave_max_lifetime: f32,
    pub ave_min_mass: f32,
    pub ave_max_mass: f32,
    pub ave_min_mass_division: f32
}

impl Info {
    pub fn new() -> Self {
        Self {
            ave_max_lifetime: 0.0,
            ave_min_mass: 0.0,
            ave_max_mass: 0.0,
            ave_min_mass_division: 0.0,
        }
    }
}
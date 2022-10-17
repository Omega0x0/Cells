pub struct Filters {
    pub max_lifetime: bool,
    pub max_mass: bool,
    pub min_mass: bool,
    pub min_mass_division: bool,
}

impl Filters {
    pub fn new() -> Self {
        Self {
            max_lifetime: false,
            max_mass: false,
            min_mass: false,
            min_mass_division: false,
        }
    }
}
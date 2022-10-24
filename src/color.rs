use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct ColorCell {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl ColorCell {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            r,
            g,
            b
        }
    }

    pub fn modify(&mut self) {
        self.r += rand::thread_rng().gen_range(-0.2..=0.2);
        self.g += rand::thread_rng().gen_range(-0.2..=0.2);
        self.b += rand::thread_rng().gen_range(-0.2..=0.2);

        if self.r < 0.0 { self.r = 0.0; }
        if self.g < 0.0 { self.r = 0.0; }
        if self.b < 0.0 { self.r = 0.0; }

        if self.r > 1.0 { self.r = 1.0; }
        if self.g > 1.0 { self.r = 1.0; }
        if self.b > 1.0 { self.r = 1.0; }
    }
}
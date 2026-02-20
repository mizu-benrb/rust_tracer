#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Default for Interval {
    fn default() -> Interval {
        Interval { min: f64::INFINITY, max: f64::NEG_INFINITY }
    }
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn combine(&self, other: &Interval) -> Interval {
        let min = if self.min <= other.min { self.min } else { other.min };
        let max = if self.max >= other.max { self.max } else { other.max };

        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        } else if x > self.max {
            return self.max;
        }
        x
    }

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
    
    pub fn is_default(&self) -> bool {
        self.min == f64::INFINITY && self.max == f64::NEG_INFINITY
    }

    pub const EMPTY: Interval = Interval { min: f64::INFINITY, max: f64::NEG_INFINITY };
    pub const UNIVERSE: Interval = Interval { min: f64::NEG_INFINITY, max: f64::INFINITY };
}
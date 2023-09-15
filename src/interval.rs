use std::ops::Range;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new() -> Interval {
        Self::empty()
    }

    pub fn empty() -> Interval {
        Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn universe() -> Interval {
        Interval {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn from(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

impl From<Range<f64>> for Interval {
    fn from(range: Range<f64>) -> Self {
        Self {
            min: range.start,
            max: range.end,
        }
    }
}

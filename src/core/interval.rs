/// Represents a one-dimensional interval [min, max].
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Interval { min, max }
    }

    pub fn contains(&self, value: f32) -> bool {
        value >= self.min && value <= self.max
    }

    pub fn overlap(&self, other: &Interval) -> Option<Interval> {
        let new_min = self.min.max(other.min);
        let new_max = self.max.min(other.max);

        if new_min <= new_max {
            Some(Interval::new(new_min, new_max))
        } else {
            None
        }
    }

    pub fn length(&self) -> f32 {
        self.max - self.min
    }

    pub fn clamp(&self, value: f32) -> f32 {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }

    pub fn expand(&self, amount: f32) -> Interval {
        Interval {
            min: self.min - amount,
            max: self.max + amount,
        }
    }
}

pub const fn universe() -> Interval {
    Interval {
        min: f32::NEG_INFINITY,
        max: f32::INFINITY,
    }
}

pub const fn empty() -> Interval {
    Interval {
        min: f32::INFINITY,
        max: f32::NEG_INFINITY,
    }
}

pub fn surround(first: &Interval, second: &Interval) -> Interval {
    let small = first.min.min(second.min);
    let big = first.max.max(second.max);
    Interval::new(small, big)
}

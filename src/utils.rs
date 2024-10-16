use std::iter::Iterator;

pub struct FloatRange {
    start: f32,
    end: f32,
    step_by: f32,
    current: f32,
}

impl FloatRange {
    pub fn new(start: f32, end: f32, step_by: f32) -> Self {
        if step_by <= 0. {
            panic!("Range must be greater than 0")
        }

        Self {
            start,
            end,
            step_by,
            current: start,
        }
    }
}

impl Iterator for FloatRange {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let next_val = self.current + self.step_by;
        if next_val >= self.end {
            return None;
        }

        self.current = next_val;
        Some(self.current)
    }
}
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct GameTimer {
    now: Instant,
    time_since: Duration,
}

impl GameTimer {
    pub fn new() -> Self {
        Self {
            now: Instant::now(),
            time_since: Duration::from_secs(0),
        }
    }

    pub fn tick(&mut self) {
        let time_since = self.now.elapsed();
        self.time_since = time_since;

        self.now = Instant::now();
    }

    pub fn delta_time(&self) -> f32 {
        self.time_since.as_secs_f32()
    }
}

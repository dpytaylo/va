use log::info;
use std::time::SystemTime;

pub struct Benchmark {
    name: &'static str,
    start_time: SystemTime,
}

impl Benchmark {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            start_time: SystemTime::now(),
        }
    }

    pub fn finish(self) {
        info!(
            "Benchmark \"{}\": {}us", // us instead of μs for a console supporting
            self.name,
            self.start_time
                .elapsed()
                .expect("invalid system clock")
                .as_micros()
        );
    }
}

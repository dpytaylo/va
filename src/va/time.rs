use std::f64::consts::PI;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct Time {
    start_time: SystemTime,
}

pub struct Now;

pub struct FromStart {
    duration: u128,
}

impl Time {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn now(&self) -> Now {
        Now {}
    }

    pub fn from_start(&self) -> FromStart {
        FromStart {
            duration: self
                .start_time
                .elapsed()
                .unwrap_or_else(|_| Duration::from_secs(0))
                .as_millis(),
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            start_time: SystemTime::now(),
        }
    }
}

impl Now {
    /// # Panic
    ///
    /// Function panics if time is invalid
    pub fn as_millis() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("invalid time")
            .as_millis()
    }
}

impl FromStart {
    pub fn as_millis(&self) -> u128 {
        self.duration
    }

    /// Max value 2*PI (use only for sin, cos functions)
    pub fn as_f32(&self) -> f32 {
        self.as_f64() as f32
    }

    /// Max value 2*PI (use only for sin, cos functions)
    pub fn as_f64(&self) -> f64 {
        (self.as_millis() as f64) % (2.0 * PI * 1_000_000.0) * 0.001 % (2.0 * PI)
    }
}

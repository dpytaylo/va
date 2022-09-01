use std::cell::Cell;
use std::time::{Duration, SystemTime};

#[derive(Clone)]
pub struct FramerateCounter {
    last_frame_time: Cell<SystemTime>,
    last_fps_time: Cell<SystemTime>,
    frames_counter: Cell<u64>,

    frame_time: Cell<u64>, // In microseconds
    fps: Cell<u64>,
}

impl FramerateCounter {
    pub fn new() -> Self {
        Self::default()
    }

    // In microseconds
    pub fn frame_time(&self) -> u64 {
        self.frame_time.get()
    }

    pub fn fps(&self) -> u64 {
        self.fps.get()
    }

    pub fn update(&self) {
        self.frame_time.set(
            self.last_frame_time
                .get()
                .elapsed()
                .unwrap_or_else(|_| Duration::from_micros(0))
                .as_micros() as u64,
        );
        self.last_frame_time.set(SystemTime::now());

        if self
            .last_fps_time
            .get()
            .elapsed()
            .unwrap_or_else(|_| Duration::from_millis(0))
            >= Duration::from_secs(1)
        {
            self.fps.set(self.frames_counter.get());

            self.last_fps_time.set(self.last_frame_time.get());
            self.frames_counter.set(0);
        }

        self.frames_counter.set(self.frames_counter.get() + 1);
    }
}

impl Default for FramerateCounter {
    fn default() -> Self {
        Self {
            last_frame_time: Cell::new(SystemTime::now()),
            last_fps_time: Cell::new(SystemTime::now()),
            frames_counter: Cell::new(0),

            frame_time: Cell::new(0),
            fps: Cell::new(0),
        }
    }
}

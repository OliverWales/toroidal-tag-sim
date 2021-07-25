use std::time::{Duration, Instant};

// Named performance counter implementation to track frames per second and
// milliseconds per frame

pub struct FPSCounter {
    name: String,
    update_period: u64,
    last_frame_time: std::time::Instant,
    elapsed_frames: u32,
}

impl FPSCounter {
    pub fn new(name: String, update_period: u64) -> FPSCounter {
        FPSCounter {
            name: name,
            update_period: update_period,
            last_frame_time: Instant::now(),
            elapsed_frames: 0,
        }
    }

    pub fn tick(&mut self) -> () {
        let elapsed_time = self.last_frame_time.elapsed();
        self.elapsed_frames += 1;

        if elapsed_time > Duration::new(self.update_period, 0) {
            println!(
                "[{}] {:.2} FPS - {:.2} ms/Frame",
                self.name,
                self.elapsed_frames as f64 / elapsed_time.as_millis() as f64 * 1000.,
                elapsed_time.as_millis() as f64 / self.elapsed_frames as f64
            );
            self.last_frame_time = Instant::now();
            self.elapsed_frames = 0;
        }
    }
}

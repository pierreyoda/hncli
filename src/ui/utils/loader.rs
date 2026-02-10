use std::time::SystemTime;

const LOADER_DOTS_UPDATE_MS: u128 = 250;

/// Used by UI components to provide an "animated" loading status text.
#[derive(Debug)]
pub struct Loader {
    /// Elapsed system time since last dots update.
    elapsed_time: SystemTime,
    /// Additional dots for text; between 1 and 3.
    current_dots: u8,
}

impl Default for Loader {
    fn default() -> Self {
        Self {
            elapsed_time: SystemTime::UNIX_EPOCH,
            current_dots: 1,
        }
    }
}

impl Loader {
    pub fn update(&mut self) {
        let now = SystemTime::now();
        let delta = now
            .duration_since(self.elapsed_time)
            .expect("Loader: duration delta should compute.");

        if delta.as_millis() < LOADER_DOTS_UPDATE_MS {
            return;
        }
        self.elapsed_time = SystemTime::now();
        self.current_dots = match self.current_dots {
            1 => 2,
            2 => 3,
            3 => 1,
            _ => unreachable!(),
        };
    }

    /// Stop and reset.
    pub fn stop(&mut self) {
        self.elapsed_time = SystemTime::UNIX_EPOCH;
        self.current_dots = 1;
    }

    pub fn text(&self) -> String {
        let mut loading_text = "Loading".to_string();
        for _ in 0..self.current_dots {
            loading_text += ".";
        }
        loading_text
    }
}

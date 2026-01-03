use crate::ui::common::UiTickScalar;

/// Used by UI components to provide debouncing capabilities,
/// for instance between key presses.
#[derive(Debug)]
pub struct Debouncer {
    /// Elapsed ticks since the latest allowed action.
    elapsed_ticks: UiTickScalar,
    /// Minimum throttling time between allowed actions. 1 tick ~= 100ms.
    throttling_min_time: UiTickScalar,
}

impl Debouncer {
    pub fn new(throttling_min_time: UiTickScalar) -> Self {
        Self {
            elapsed_ticks: 0,
            throttling_min_time,
        }
    }

    pub fn reset(&mut self) {
        self.elapsed_ticks = 0;
    }

    pub fn tick(&mut self, elapsed_ticks: UiTickScalar) {
        self.elapsed_ticks = self.elapsed_ticks.wrapping_add(elapsed_ticks);
    }

    pub fn is_action_allowed(&mut self) -> bool {
        let allowed = self.elapsed_ticks >= self.throttling_min_time;
        if allowed {
            self.reset();
        }
        allowed
    }
}

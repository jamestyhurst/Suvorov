//! Core placeholder: a bare tick counter, standing in for the simulation clock.

/// A monotonically increasing tick counter.
#[derive(Debug, Default)]
pub struct Clock {
    tick: u64,
}

impl Clock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }

    pub fn advance(&mut self) {
        self.tick += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::Clock;

    #[test]
    fn clock_starts_at_zero_and_advances_one_tick() {
        let mut clock = Clock::new();
        assert_eq!(clock.tick(), 0);
        clock.advance();
        assert_eq!(clock.tick(), 1);
    }
}
